import React, { createContext, useState, useContext, useEffect, ReactNode } from 'react';
import LocalStorage from '../LocalStorage';

type AuthTokens = { token: string | null; refreshToken: string | null };

interface AuthContextType {
  isAuthenticating: boolean;
  setIsAuthenticating: (isAuthenticating: boolean) => void;
  isLoggedIn: boolean;
  getTokens: () => Promise<AuthTokens>;
  saveTokens: (token: string, refreshToken: string) => Promise<void>;
  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | null>(null);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [isAuthenticating, setIsAuthenticating] = useState(true);
  const [isLoggedIn, setIsLoggedIn] = useState(true);

  const getTokens = async () => {
    const token = await LocalStorage.getItem('token');
    const refreshToken = await LocalStorage.getItem('refreshToken');
    return { token, refreshToken };
  };

  useEffect(() => {
    getTokens()
      .then(({ token }) => setIsLoggedIn(!!token))
      .catch((e) => {
        console.error('Failed to load token from LocalStorage', e);
        setIsLoggedIn(false);
      })
      .finally(() => setIsAuthenticating(false));
  }, []);

  const saveTokens = async (newToken: string, newRefreshToken: string) => {
    try {
      await LocalStorage.setItem('token', newToken);
      await LocalStorage.setItem('refreshToken', newRefreshToken);
      setIsLoggedIn(true);
    } catch (e) {
      console.error('Failed to save tokens to LocalStorage', e);
      setIsLoggedIn(false);
    } finally {
      setIsAuthenticating(false);
    }
  };

  const logout = async () => {
    try {
      await LocalStorage.removeItem('token');
      await LocalStorage.removeItem('refreshToken');
    } catch (e) {
      console.error('Failed to remove tokens from LocalStorage', e);
    } finally {
      setIsLoggedIn(false);
      setIsAuthenticating(false);
    }
  };

  const authContextValue = {
    isAuthenticating,
    setIsAuthenticating,
    isLoggedIn,
    getTokens,
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
