import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './styles/globals.css';
import './i18n';
import { getCurrentWindow } from '@tauri-apps/api/window';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

// Avoid initial blank-frame flicker: show the window after first paint.
const currentWindow = getCurrentWindow();
requestAnimationFrame(() => {
  requestAnimationFrame(() => {
    currentWindow.show().catch(() => {
      // ignore (e.g., during non-tauri contexts)
    });
  });
});
