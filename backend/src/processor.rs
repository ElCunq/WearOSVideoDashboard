use std::process::Command;
use std::path::{Path, PathBuf};
use tokio::fs;
use chrono::Utc;
use serde_json::json;
use std::io::{Read, Seek, SeekFrom};

pub async fn generate_preview(file_id: &str) -> Result<String, String> {
    let input_path = format!("uploads/{}", file_id);
    let output_path = format!("static/previews/{}.mp4", file_id);
    
    fs::create_dir_all("static/previews").await.map_err(|e| e.to_string())?;

    let final_input = if is_pkg(&input_path) {
        extract_video_from_pkg(&input_path, file_id).await?
    } else {
        PathBuf::from(&input_path)
    };

    let status = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-i", final_input.to_str().unwrap(),
            "-vf", "scale=640:-2",
            "-c:v", "libx264",
            "-preset", "ultrafast",
            "-crf", "28",
            "-an",
            "-movflags", "faststart",
            &output_path,
        ])
        .status()
        .map_err(|e| e.to_string())?;

    if !status.success() {
        return Err("Önizleme oluşturulamadı. Dosya bozuk veya desteklenmeyen bir format olabilir.".to_string());
    }

    Ok(format!("/static/previews/{}.mp4", file_id))
}

pub async fn process_video(
    file_id: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    pixel_shift: bool,
) -> Result<(), String> {
    let input_path = format!("uploads/{}", file_id);
    let output_path = "static/loop.mp4";
    let version_path = "static/version.json";

    if !Path::new(&input_path).exists() {
        return Err("Orijinal dosya bulunamadı.".to_string());
    }

    let final_input = if is_pkg(&input_path) {
        extract_video_from_pkg(&input_path, file_id).await?
    } else {
        PathBuf::from(&input_path)
    };

    let scale = if pixel_shift { "454:454" } else { "450:450" };
    // Scale to 640 width first (matches preview) then crop using frontend coordinates
    let crop_filter = format!("scale=640:-1,crop={}:{}:{}:{},scale={}", width, height, x, y, scale);

    let status = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-i", final_input.to_str().unwrap(),
            "-vf", &crop_filter,
            "-c:v", "libx264",
            "-profile:v", "baseline",
            "-level", "3.0",
            "-an",
            "-pix_fmt", "yuv420p",
            "-r", "30",
            "-b:v", "1.5M",
            "-maxrate", "2M",
            "-bufsize", "2M",
            output_path,
        ])
        .status()
        .map_err(|e| e.to_string())?;

    if !status.success() {
        return Err("Video işleme başarısız oldu.".to_string());
    }

    let version_info = json!({
        "version": Utc::now().timestamp(),
        "video_url": "/static/loop.mp4",
        "brightness": 0.6
    });

    fs::write(version_path, serde_json::to_string_pretty(&version_info).unwrap())
        .await
        .map_err(|e| e.to_string())?;

    fs::write(version_path, serde_json::to_string_pretty(&version_info).unwrap())
        .await
        .map_err(|e| e.to_string())?;

    // No longer removing input_path to allow for library management
    Ok(())
}

fn is_pkg(path: &str) -> bool {
    match std::fs::File::open(path) {
        Ok(mut file) => {
            let mut buf = [0u8; 4];
            if file.read_exact(&mut buf).is_ok() {
                let root_len = u32::from_le_bytes(buf);
                return root_len > 0 && root_len < 2048;
            }
        }
        _ => {}
    }
    false
}

async fn extract_video_from_pkg(path: &str, file_id: &str) -> Result<PathBuf, String> {
    let mut file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    
    let mut buf = [0u8; 4];
    file.read_exact(&mut buf).map_err(|e| e.to_string())?;
    let root_len = u32::from_le_bytes(buf) as usize;
    if root_len > 10000 { return Err("Geçersiz PKG formatı.".to_string()); }
    
    let mut root_path_buf = vec![0u8; root_len];
    file.read_exact(&mut root_path_buf).map_err(|e| e.to_string())?;
    
    file.read_exact(&mut buf).map_err(|e| e.to_string())?;
    let num_files = u32::from_le_bytes(buf);
    
    let mut video_file_info = None;
    let mut file_names = Vec::new();

    for _ in 0..num_files {
        file.read_exact(&mut buf).map_err(|e| e.to_string())?;
        let name_len = u32::from_le_bytes(buf) as usize;
        let mut name_buf = vec![0u8; name_len];
        file.read_exact(&mut name_buf).map_err(|e| e.to_string())?;
        let name = String::from_utf8_lossy(&name_buf).to_string();
        
        file.read_exact(&mut buf).map_err(|e| e.to_string())?;
        let offset = u32::from_le_bytes(buf);
        file.read_exact(&mut buf).map_err(|e| e.to_string())?;
        let length = u32::from_le_bytes(buf);
        
        file_names.push(name.clone());
        
        let lower_name = name.to_lowercase();
        if lower_name.ends_with(".mp4") || lower_name.ends_with(".webm") || lower_name.ends_with(".mkv") {
            // Priority to larger video files
            if let Some((_, _, old_len)) = video_file_info {
                if length > old_len {
                    video_file_info = Some((name, offset, length));
                }
            } else {
                video_file_info = Some((name, offset, length));
            }
        }
    }

    if let Some((_, offset, length)) = video_file_info {
        file.seek(SeekFrom::Start(offset as u64)).map_err(|e| e.to_string())?;
        let mut data = vec![0u8; length as usize];
        file.read_exact(&mut data).map_err(|e| e.to_string())?;
        
        let out_path = format!("uploads/{}_extracted.mp4", file_id);
        fs::write(&out_path, data).await.map_err(|e| e.to_string())?;
        return Ok(PathBuf::from(out_path));
    }

    let files_list = file_names.join(", ");
    Err(format!(
        "PKG içerisinde video dosyası bulunamadı. Bu bir 'Scene' duvar kağıdı olabilir. İçerik: [{}]",
        if files_list.len() > 100 { format!("{}...", &files_list[..100]) } else { files_list }
    ))
}
