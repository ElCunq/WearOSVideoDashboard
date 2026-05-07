# Yol Haritası ve "Power-User" Özellikleri

Sistemin geliştirilme aşamaları ve teknik detayları.

## Roadmap

1.  **Aşama: Backend Hazırlığı**
    - FastAPI iskeletinin kurulması.
    - FFmpeg wrapper fonksiyonunun yazılması.
    - Static dosya servisinin (Nginx veya FastAPI Static) yapılandırılması.

2.  **Aşama: Frontend Geliştirme**
    - Vite + React projesinin oluşturulması.
    - Video transform logic'inin (Canvas veya CSS) kurulması.
    - Kırpma koordinatlarının Backend'e entegrasyonu.

3.  **Aşama: Entegrasyon ve Test**
    - Uçtan uca video yükleme, işleme ve izleme testleri.
    - Isı yönetimi ve bitrate kontrollerinin doğrulanması.

4.  **Aşama: WearOS Client Geliştirme**
    - Minimal Android projesinin oluşturulması.
    - Player logic ve Pixel Shifting entegrasyonu.
    - Otomatik senkronizasyon (Auto-sync) mekanizmasının kurulması.

## Power-User Özellikleri

### A. AMOLED Koruma (Pixel Shifting)
- **Problem**: Sabit görüntülerin AMOLED ekranlarda "burn-in" yapması.
- **Çözüm**:
    - Video 450x450 yerine 454x454 olarak kırpılır.
    - Saat tarafındaki player, her 5 dakikada bir videonun `(top, left)` değerlerini ±2 piksel kaydırır.
    - Bu sayede aynı pikseller sürekli aynı renkte kalmaz.

### B. Isı Yönetimi (Thermal Optimization)
- Saatlerde işlemci gücü sınırlı olduğundan, encode parametreleri kritiktir.
- **Max Bitrate**: 1.5 - 2 Mbps.
- **Framerate**: 30 FPS (Gereksiz karelerin elenmesi).

### C. Akıllı Polling
- Saat tarafı `version.json` dosyasını kontrol eder.
- Eğer `version` değeri lokaldeki ile aynıysa indirme yapılmaz.
- Bu, pil ömrünü korur ve gereksiz ağ trafiğini engeller.
