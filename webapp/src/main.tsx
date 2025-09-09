import React from 'react';
import ReactDOM from 'react-dom/client';
import { AuthProvider } from '@shared/contexts/AuthContext';
import App from './App.tsx';
import LocalStorage from './LocalStorage';
import './index.css';
import { setDocumentTitle } from './setDocumentTitle';
setDocumentTitle();

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <AuthProvider storage={LocalStorage}>
      <App />
    </AuthProvider>
  </React.StrictMode>,
);
