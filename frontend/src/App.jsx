import React, { useState, useRef, useEffect } from 'react';
import './App.css';

const SCREEN_SIZE = 450;

function App() {
  const [videoSrc, setVideoSrc] = useState(null);
  const [fileId, setFileId] = useState(null);
  const [pos, setPos] = useState({ x: 0, y: 0 });
  const [zoom, setZoom] = useState(1);
  const [rotate, setRotate] = useState(0);
  const [pixelShift, setPixelShift] = useState(true);
  const [isDragging, setIsDragging] = useState(false);
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 });
  const [libraryFiles, setLibraryFiles] = useState([]);
  const [status, setStatus] = useState('ready');

  const videoRef = useRef(null);

  useEffect(() => {
    fetchLibrary();
  }, []);

  const fetchLibrary = async () => {
    try {
      const response = await fetch('/files');
      if (response.ok) {
        const data = await response.json();
        setLibraryFiles(data);
      }
    } catch (err) {
      console.error('Kütüphane yüklenemedi:', err);
    }
  };

  const deleteFile = async (id, e) => {
    e.stopPropagation();
    if (!window.confirm('Bu videoyu silmek istediğinize emin misiniz?')) return;
    
    try {
      const response = await fetch(`/files/${id}`, { method: 'DELETE' });
      if (response.ok) {
        fetchLibrary();
        if (fileId === id) {
          setVideoSrc(null);
          setFileId(null);
        }
      }
    } catch (err) {
      alert('Silme hatası');
    }
  };

  const handleFileChange = async (e) => {
    const file = e.target.files[0];
    if (!file) return;

    setStatus('uploading');
    const formData = new FormData();
    formData.append('video', file);

    try {
      const response = await fetch('/upload', {
        method: 'POST',
        body: formData,
      });
      
      if (!response.ok) {
        throw new Error(await response.text());
      }

      const data = await response.json();
      setFileId(data.file_id);
      setVideoSrc(data.preview_url);
      fetchLibrary();
      setStatus('ready');
    } catch (err) {
      console.error(err);
      alert('Yükleme hatası: ' + err.message);
      setStatus('ready');
    }
  };

  const handleMouseDown = (e) => {
    setIsDragging(true);
    setDragStart({ x: e.clientX - pos.x, y: e.clientY - pos.y });
  };

  const handleMouseMove = (e) => {
    if (!isDragging) return;
    setPos({
      x: e.clientX - dragStart.x,
      y: e.clientY - dragStart.y,
    });
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  const handleSave = async () => {
    if (!fileId || !videoRef.current) return;

    setStatus('processing');
    
    const relX = (-pos.x) / zoom;
    const relY = (-pos.y) / zoom;
    const relW = SCREEN_SIZE / zoom;
    const relH = SCREEN_SIZE / zoom;

    const payload = {
      file_id: fileId,
      x: Math.max(0, Math.round(relX)),
      y: Math.max(0, Math.round(relY)),
      width: Math.round(relW),
      height: Math.round(relH),
      pixel_shift: pixelShift,
    };

    try {
      const response = await fetch('/process', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });
      if (response.ok) {
        alert('Video başarıyla işlendi!');
        // Keep the editor open but update status
      } else {
        const err = await response.text();
        alert('İşleme hatası: ' + err);
      }
    } catch (err) {
      console.error(err);
      alert('Ağ hatası.');
    } finally {
      setStatus('ready');
    }
  };

  useEffect(() => {
    window.addEventListener('mouseup', handleMouseUp);
    return () => window.removeEventListener('mouseup', handleMouseUp);
  }, []);

  return (
    <div className="app-container" onMouseMove={handleMouseMove}>
      <header className="header">
        <h1>Galaxy Watch 4 Classic</h1>
        <p>Gelişmiş Video Dashboard</p>
      </header>

      <div className="main-layout">
        {/* Kütüphane Paneli */}
        <aside className="library-panel">
          <div className="library-header">
            <h3>MEDYA KÜTÜPHANESİ</h3>
            <div className="upload-btn-mini">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
              <input type="file" onChange={handleFileChange} />
            </div>
          </div>
          <div className="library-scroll">
            {libraryFiles.map(file => (
              <div 
                key={file.id} 
                className={`library-item ${fileId === file.id ? 'active' : ''}`}
                onClick={() => { setFileId(file.id); setVideoSrc(file.preview_url); }}
              >
                <div className="item-preview">
                  <video src={file.preview_url} muted onMouseOver={e => e.target.play()} onMouseOut={e => {e.target.pause(); e.target.currentTime = 0;}} />
                </div>
                <div className="item-info">
                  <span className="item-name">{file.name}</span>
                </div>
                <button className="delete-btn" onClick={(e) => deleteFile(file.id, e)}>
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
                </button>
              </div>
            ))}
            {libraryFiles.length === 0 && <div className="empty-library">Henüz video yok</div>}
          </div>
        </aside>

        <main className="editor-container">
          <div className="watch-frame"></div>
          <div className="mask-overlay"></div>
          
          <div className="screen-area">
            {!videoSrc && status === 'ready' && (
              <div className="upload-overlay">
                <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                  <polyline points="17 8 12 3 7 8"></polyline>
                  <line x1="12" y1="3" x2="12" y2="15"></line>
                </svg>
                <span style={{marginTop: '12px', fontSize: '0.7rem', fontWeight: '800', letterSpacing: '0.1em'}}>VİDEO YÜKLE</span>
                <input type="file" onChange={handleFileChange} />
              </div>
            )}

            {status === 'uploading' && (
              <div className="upload-overlay">
                <div className="spinner"></div>
                <span style={{marginTop: '12px', fontSize: '0.7rem'}}>YÜKLENİYOR...</span>
              </div>
            )}

            {videoSrc && (
              <div 
                className="video-wrapper" 
                style={{ 
                  left: pos.x, 
                  top: pos.y, 
                  transform: `scale(${zoom}) rotate(${rotate}deg)` 
                }}
                onMouseDown={handleMouseDown}
              >
                <video ref={videoRef} key={videoSrc} src={videoSrc} autoPlay loop muted />
              </div>
            )}
          </div>
        </main>

        {/* Side Precision Controls */}
        {videoSrc && (
          <div className="precision-controls">
            <div className="precision-group">
              <label>X KONUMU</label>
              <input 
                type="number" 
                value={Math.round(pos.x)} 
                onChange={(e) => setPos({...pos, x: parseInt(e.target.value) || 0})} 
              />
              <div className="vertical-slider-container">
                <input 
                  type="range" 
                  min="-2000" 
                  max="2000" 
                  value={pos.x} 
                  onChange={(e) => setPos({...pos, x: parseInt(e.target.value)})} 
                  className="vertical-slider"
                />
              </div>
            </div>
            <div className="precision-group">
              <label>Y KONUMU</label>
              <input 
                type="number" 
                value={Math.round(pos.y)} 
                onChange={(e) => setPos({...pos, y: parseInt(e.target.value) || 0})} 
              />
              <div className="vertical-slider-container">
                <input 
                  type="range" 
                  min="-2000" 
                  max="2000" 
                  value={pos.y} 
                  onChange={(e) => setPos({...pos, y: parseInt(e.target.value)})} 
                  className="vertical-slider"
                />
              </div>
            </div>
          </div>
        )}
      </div>

      {videoSrc && (
        <section className="controls">
          <div className="control-group">
            <div style={{display:'flex', justifyContent:'space-between'}}>
              <label>YAKINLAŞTIRMA</label>
              <span style={{fontSize:'0.65rem', fontWeight:'700'}}>{(zoom * 100).toFixed(0)}%</span>
            </div>
            <input 
              type="range" 
              className="slider" 
              min="0.1" 
              max="4" 
              step="0.01" 
              value={zoom} 
              onChange={(e) => setZoom(parseFloat(e.target.value))} 
            />
          </div>

          <div className="control-group">
            <div style={{display:'flex', justifyContent:'space-between'}}>
              <label>DÖNDÜRME</label>
              <span style={{fontSize:'0.65rem', fontWeight:'700'}}>{rotate}°</span>
            </div>
            <input 
              type="range" 
              className="slider" 
              min="0" 
              max="360" 
              value={rotate} 
              onChange={(e) => setRotate(parseInt(e.target.value))} 
            />
          </div>

          <label className="pixel-shift-toggle">
            <input 
              type="checkbox" 
              checked={pixelShift} 
              onChange={(e) => setPixelShift(e.target.checked)} 
            />
            <span>AMOLED YANMA KORUMASI AKTİF</span>
          </label>

          <div className="button-group">
            <button 
              className="btn btn-secondary" 
              onClick={() => {setVideoSrc(null); setFileId(null);}}
            >
              İPTAL
            </button>
            <button 
              className="btn btn-primary" 
              onClick={handleSave}
              disabled={status === 'processing'}
            >
              {status === 'processing' ? 'İŞLENİYOR...' : 'SAATE GÖNDER'}
            </button>
          </div>
        </section>
      )}
    </div>
  );
}

export default App;
