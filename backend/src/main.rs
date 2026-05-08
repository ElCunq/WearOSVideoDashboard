mod processor;

use axum::{
    extract::{Multipart, DefaultBodyLimit},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use uuid::Uuid;
use tokio::fs;

#[derive(Serialize, Deserialize)]
struct ProcessRequest {
    file_id: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    pixel_shift: bool,
}

#[tokio::main]
async fn main() {
    // Add panic hook to see errors in logs
    std::panic::set_hook(Box::new(|panic_info| {
        println!(">>> CRITICAL ERROR (PANIC): {}", panic_info);
    }));

    println!(">>> WearOS Video Backend Starting Up...");
    
    // Ensure essential directories exist
    fs::create_dir_all("uploads").await.unwrap();
    fs::create_dir_all("static/previews").await.unwrap();

    let app = Router::new()
        // API Routes
        .route("/upload", post(upload_video))
        .route("/files", get(list_files))
        .route("/files/:id", axum::routing::delete(delete_file))
        .route("/process", post(process_video_handler))
        // Serve Processed Videos & Previews
        .nest_service("/static", ServeDir::new("static"))
        // Serve Built Frontend
        .nest_service("/", ServeDir::new("dist").fallback(ServeFile::new("dist/index.html")))
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::max(500 * 1024 * 1024));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap();
    
    println!("Server running on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn list_files() -> impl IntoResponse {
    let mut files = Vec::new();
    if let Ok(mut entries) = fs::read_dir("uploads").await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let filename = entry.file_name().into_string().unwrap_or_default();
            if filename.contains("__") {
                let parts: Vec<&str> = filename.splitn(2, "__").collect();
                if parts.len() == 2 {
                    files.push(serde_json::json!({
                        "id": parts[0],
                        "name": parts[1],
                        "preview_url": format!("/static/previews/{}.mp4", parts[0])
                    }));
                }
            }
        }
    }
    Json(files)
}

async fn delete_file(axum::extract::Path(id): axum::extract::Path<String>) -> impl IntoResponse {
    // Find file in uploads that starts with id
    if let Ok(mut entries) = fs::read_dir("uploads").await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let filename = entry.file_name().into_string().unwrap_or_default();
            if filename.starts_with(&id) {
                let _ = fs::remove_file(entry.path()).await;
                // Also remove preview
                let _ = fs::remove_file(format!("static/previews/{}.mp4", id)).await;
                return (StatusCode::OK, "Silindi").into_response();
            }
        }
    }
    (StatusCode::NOT_FOUND, "Dosya bulunamadı").into_response()
}

async fn upload_video(mut multipart: Multipart) -> impl IntoResponse {
    let mut file_id = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("unknown").to_string();
        let file_name = field.file_name().unwrap_or("video.mp4").to_string();
        
        if name == "video" {
            let id = Uuid::new_v4().to_string();
            match field.bytes().await {
                Ok(data) => {
                    // Save as ID__Filename to preserve metadata without DB
                    let path = format!("uploads/{}__{}", id, file_name);
                    if let Err(e) = fs::write(&path, data).await {
                        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Kaydetme hatası: {}", e)).into_response();
                    }
                    file_id = Some(id);
                }
                Err(e) => return (StatusCode::BAD_REQUEST, format!("Dosya okuma hatası: {}", e)).into_response(),
            }
        }
    }

    if let Some(id) = file_id {
        // Need to find the exact filename for preview generation
        let mut actual_id = id.clone();
        if let Ok(mut entries) = fs::read_dir("uploads").await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let filename = entry.file_name().into_string().unwrap_or_default();
                if filename.starts_with(&id) {
                    actual_id = filename; // This is the full filename like id__name.mp4
                    break;
                }
            }
        }

        match processor::generate_preview(&actual_id).await {
            Ok(preview_url) => (StatusCode::OK, Json(serde_json::json!({ 
                "file_id": id, 
                "preview_url": preview_url 
            }))).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Önizleme hatası: {}", e)).into_response(),
        }
    } else {
        (StatusCode::BAD_REQUEST, "Dosya bulunamadı").into_response()
    }
}

async fn process_video_handler(Json(payload): Json<ProcessRequest>) -> impl IntoResponse {
    // Find the full filename in uploads
    let mut actual_filename = payload.file_id.clone();
    if let Ok(mut entries) = fs::read_dir("uploads").await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let filename = entry.file_name().into_string().unwrap_or_default();
            if filename.starts_with(&payload.file_id) {
                actual_filename = filename;
                break;
            }
        }
    }

    match processor::process_video(
        &actual_filename,
        payload.x,
        payload.y,
        payload.width,
        payload.height,
        payload.pixel_shift,
    ).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({ "status": "success" }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}
