# Frontend Spesifikasyonu

Kullanıcının videoyu yüklediği ve saat ekranına göre hizaladığı katman.

## Teknoloji Yığını
- **Framework**: React (Vite ile).
- **Styling**: Vanilla CSS (Modern CSS özellikleri ile).
- **State Management**: React Hooks (useState, useEffect).

## Özellikler
1.  **Circular Mask Preview**:
    - Ortada 450x450px sabit bir daire.
    - Dairenin dışı yarı saydam (overlay) olacak şekilde karartılmış.
2.  **Transform Tools**:
    - **Drag**: Videoyu fare veya dokunmatik ile dairesel alanın altında sürükleme.
    - **Zoom**: Slider veya mouse wheel ile videoyu büyütme/küçültme.
    - **Rotate**: Videoyu döndürme (isteğe bağlı).
3.  **Coordinate Capture**:
    - "Kaydet" butonuna basıldığında:
        - Orijinal video boyutları baz alınarak kırpma koordinatları (x, y, w, h) hesaplanır.
        - Bu veriler Backend'e JSON olarak gönderilir.

## Kullanıcı Deneyimi (UX)
- Modern, karanlık tema (Dark Mode).
- Akıcı animasyonlar ve mikro-etkileşimler.
- Video yüklendiğinde otomatik olarak dairesel merkeze odaklanma.
