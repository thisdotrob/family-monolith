import React, { createContext, useState, useContext, ReactNode, useEffect } from 'react';
import { useReactiveVar, useApolloClient } from '@apollo/client';
import { isRefreshingTokenVar } from '../api/state';
import { saveTokens as saveTokensToStorage, logout as logoutUser } from '../auth';

interface AuthContextType {
  token: string | null;
  isRefreshingToken: boolean;
  setIsRefreshingToken: (isRefreshing: boolean) => void;
  saveTokens: (newToken: string, newRefreshToken: string) => void;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType | null>(null);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const client = useApolloClient();
  const [token, setToken] = useState<string | null>(() => localStorage.getItem('token'));
  const isRefreshingToken = useReactiveVar(isRefreshingTokenVar);

  const saveTokens = (newToken: string, newRefreshToken: string) => {
    saveTokensToStorage(client, newToken, newRefreshToken);
    setToken(newToken);
  };

  const logout = () => {
    logoutUser(client);
    setToken(null);
  };
  
  useEffect(() => {
    const handleStorageChange = () => {
      setToken(localStorage.getItem('token'));
    };
    window.addEventListener('storage', handleStorageChange);
    return () => {
      window.removeEventListener('storage', handleStorageChange);
    };
  }, []);


  const authContextValue = {
    token,
    isRefreshingToken,
    setIsRefreshingToken: isRefreshingTokenVar,
    saveTokens,
    logout,
  };

  return (
    <AuthContext.Provider value={authContextValue}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};
