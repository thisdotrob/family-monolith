import React from 'react';
import { AuthProvider } from '../src/contexts/AuthContext';
import App from '../src/App';

export default function RootLayout() {
  return (
    <AuthProvider>
      <App />
    </AuthProvider>
  );
}
