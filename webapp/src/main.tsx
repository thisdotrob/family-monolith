import React from 'react';
import ReactDOM from 'react-dom/client';
import { ApolloProvider } from '@apollo/client';
import { AuthProvider, useAuth } from './contexts/AuthContext';
import App from './App.tsx';
import './index.css';
import { createApolloClient } from './api/apollo.ts';

const Main = () => {
  // We need to be inside AuthProvider to use the useAuth hook
  const { setIsRefreshing, logout } = useAuth();
  const client = createApolloClient(setIsRefreshing, logout);

  return (
    <ApolloProvider client={client}>
      <App />
    </ApolloProvider>
  );
};

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <AuthProvider>
      <Main />
    </AuthProvider>
  </React.StrictMode>
);
