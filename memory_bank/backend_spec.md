# Backend Spesifikasyonu (Rust / Axum)

Backend, videonun mutfağıdır ve tüm işleme süreçlerini yönetir. "Aşırı hafif" (extremely lightweight) olması için Rust dili seçilmiştir.

## Teknoloji Yığını
- **Dil**: Rust.
- **Framework**: Axum (Tokio üzerinde çalışan, yüksek performanslı ve hafif bir web framework).
- **Video İşleme**: FFmpeg (std::process::Command ile çağrılır).
- **Dosya Sistemi**: Yerel depolama (Static files).

## FFmpeg Pipeline
Kullanılan ana komut:
```bash
ffmpeg -i upload.mp4 -vf "crop=w:h:x:y,scale=450:450" -c:v libx264 -profile:v baseline -level 3.0 -an -pix_fmt yuv420p -r 30 processed.mp4
```

### Optimizasyonlar
- `-an`: Sesin kaldırılması (CPU tasarrufu).
- `-profile:v baseline`: Saat donanımı için düşük yoğunluklu dekodlama.
- `bitrate`: 1.5 - 2 Mbps sınırı (Isı yönetimi).
- `Pixel Shifting`: AMOLED koruması için 454x454 kırpma seçeneği.

## Veri Yapısı (version.json)
```json
{
  "version": 1715012345,
  "video_url": "http://sunucu_ip/static/loop.mp4",
  "brightness": 0.6
}
```

## API Endpoint'leri
1. `POST /upload`: Orijinal videoyu alır.
2. `POST /process`: Koordinatları (x, y, w, h) alır ve FFmpeg işlemini başlatır.
3. `GET /status`: İşleme durumunu döndürür.
4. `GET /static/*`: İşlenen videoyu ve `version.json` dosyasını sunar.

## Neden Rust?
- **Minimal Bellek Kullanımı**: Python interpreter gerektirmez.
- **Tek Binary**: Dağıtımı ve çalıştırılması çok kolaydır.
- **Hız**: Video işleme koordinasyonu ve API yanıtları için en yüksek performansı sunar.
