# WearOS Client Specification

Bu doküman, Galaxy Watch 4 Classic ve benzeri dairesel ekranlı WearOS cihazlar için geliştirilecek olan "Ultra-Lightweight" video oynatıcı uygulamasının teknik detaylarını içerir.

## 1. Temel Felsefe
- **Minimum Bağımlılık**: Uygulama boyutu ve RAM kullanımı en düşük seviyede tutulacaktır.
- **Batarya Dostu**: Gereksiz CPU/Ağ kullanımı engellenecektir.
- **Kesintisiz Döngü**: Videolar arasında atlama olmadan sonsuz döngü (seamless loop) sağlanacaktır.

## 2. Teknik Yığın (Tech Stack)
- **Platform**: Android WearOS (Min SDK 28+)
- **Dil**: Kotlin
- **Video Motoru**: `ExoPlayer` (Basit ve stabil olduğu için) veya `MediaPlayer` (Eğer ultra-hafiflik istenirse).
- **Networking**: `OkHttp` veya standart `HttpURLConnection`.

## 3. Temel Fonksiyonlar

### A. Akıllı Senkronizasyon (Smart Polling)
Uygulama arka planda veya başlatıldığında şu adımları izler:
1.  `GET /static/version.json` adresini kontrol eder.
2.  Gelen `version` değeri, cihazdaki son kaydedilen sürümden büyükse:
    -   `GET /static/loop.mp4` dosyasını indirir.
    -   Dosyayı dâhili depolama alanına (`Internal Storage`) kaydeder.
    -   Sürüm numarasını günceller.
3.  Eğer sürüm aynıysa ağ trafiği oluşturmaz.

### B. Oynatıcı Mantığı
- **Fullscreen**: Status bar ve navigasyon gizlenerek video tam ekran oynatılır.
- **Seamless Loop**: ExoPlayer'ın `RepeatMode.ALL` özelliği kullanılarak kare atlamasız döngü sağlanır.
- **Hardware Acceleration**: GPU üzerinden donanım hızlandırmalı dekoder zorlanır.

### C. AMOLED Koruma (Pixel Shifting)
Ekran yanmasını (burn-in) önlemek için:
- Video 454x454 çözünürlükte indirilir.
- Ekran alanı 450x450'dir.
- Uygulama, her 5 dakikada bir videonun `X` ve `Y` ofsetlerini rastgele ±2 piksel kaydırır.

## 4. Dosya Yapısı (Öngörülen)
```
com.wearos.videodashboard
├── MainActivity.kt (Görsel arayüz ve oynatıcı)
├── SyncWorker.kt (Arka plan senkronizasyon yöneticisi)
├── VideoManager.kt (Dosya indirme ve versiyon kontrolü)
└── model/
    └── VersionInfo.kt
```

## 5. Güç Tasarrufu Stratejisi
- Cihazın şarjı %10'un altına düştüğünde senkronizasyon durdurulur.
- Ekran kapandığında (Always-on Display modu hariç) video duraklatılır.

## 6. Kurulum ve Dağıtım
- Uygulama minimal bir APK olarak derlenir.
- ADB üzerinden veya yerel ağ üzerinden saate yüklenir.
- Sunucu IP adresi uygulama ayarlarından veya bir `config` dosyasından okunur.
