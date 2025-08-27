import React from 'react';
import { AuthProvider } from '../src/contexts/AuthContext';
import App from '../src/App.tsx';

export default function RootLayout() {
  return (
    <AuthProvider>
      <App />
    </AuthProvider>
  );
}
