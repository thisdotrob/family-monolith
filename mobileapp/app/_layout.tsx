import React from 'react';
import { AuthProvider } from '@shared/contexts/AuthContext';
import App from '../src/App';
import LocalStorage from '../src/LocalStorage';

export default function RootLayout() {
  return (
    <AuthProvider storage={LocalStorage}>
      <App />
    </AuthProvider>
  );
}
