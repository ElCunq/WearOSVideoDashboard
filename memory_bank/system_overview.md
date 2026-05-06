# WearOS Video Management System - Sistem Özeti

Bu proje, Wear OS cihazlar (dairesel ekranlı saatler) için optimize edilmiş video içeriklerinin yönetimini, işlenmesini ve dağıtımını sağlayan uçtan uca bir sistemdir.

## Mimari Katmanlar

1.  **Web Dashboard (Frontend)**: Kullanıcı arayüzü. Videoların yüklenmesi ve dairesel maskeye göre kadrajlanması.
2.  **API & İşleme Birimi (Backend)**: Videoların FFmpeg ile işlenmesi, koordinatlara göre kırpılması ve Wear OS uyumlu hale getirilmesi.
3.  **Dağıtım ve Senkronizasyon (Storage)**: İşlenen videoların ve sürüm bilgilerinin (`version.json`) servis edilmesi.
4.  **Watch Client (APK)**: Saatte çalışan, yeni videoları kontrol eden ve oynatan uygulama.

## Temel Özellikler

- **Dairesel Kadrajlama**: 450x450 piksel dairesel maske ile önizleme.
- **FFmpeg Optimizasyonu**: Baseline profile, sese veda (-an), düşük bitrate (1.5-2 Mbps).
- **AMOLED Dostu**: 454x454 kırpma ile "Pixel Shifting" desteği.
- **Akıllı Polling**: Sadece sürüm değiştiğinde indirme yapan verimli senkronizasyon.
