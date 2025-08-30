import { createContext, useState, useContext, useEffect } from 'react';
import type { ReactNode } from 'react';

export type AuthTokens = { token: string | null; refreshToken: string | null };

export interface StorageAdapter {
  setItem: (key: string, value: string) => Promise<void> | void;
  getItem: (key: string) => Promise<string | null> | string | null;
  removeItem: (key: string) => Promise<void> | void;
}

interface AuthContextType {
  isAuthenticating: boolean;
  setIsAuthenticating: (isAuthenticating: boolean) => void;
  isLoggedIn: boolean;
  getTokens: () => Promise<AuthTokens>;
  saveTokens: (token: string, refreshToken: string) => Promise<void>;
  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | null>(null);

export const AuthProvider = ({ children, storage }: { children: ReactNode; storage: StorageAdapter }) => {
  const [isAuthenticating, setIsAuthenticating] = useState(true);
  const [isLoggedIn, setIsLoggedIn] = useState(true);

  const getTokens = async () => {
    const token = await Promise.resolve(storage.getItem('token'));
    const refreshToken = await Promise.resolve(storage.getItem('refreshToken'));
    return { token, refreshToken };
  };

  useEffect(() => {
    getTokens()
      .then(({ token }) => setIsLoggedIn(!!token))
      .catch((e) => {
        console.error('Failed to load token from storage', e);
        setIsLoggedIn(false);
      })
      .finally(() => setIsAuthenticating(false));
  }, []);

  const saveTokens = async (newToken: string, newRefreshToken: string) => {
    try {
      await Promise.resolve(storage.setItem('token', newToken));
      await Promise.resolve(storage.setItem('refreshToken', newRefreshToken));
      setIsLoggedIn(true);
    } catch (e) {
      console.error('Failed to save tokens to storage', e);
      setIsLoggedIn(false);
    } finally {
      setIsAuthenticating(false);
    }
  };

  const logout = async () => {
    try {
      await Promise.resolve(storage.removeItem('token'));
      await Promise.resolve(storage.removeItem('refreshToken'));
    } catch (e) {
      console.error('Failed to remove tokens from storage', e);
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

export const useAuth = (): AuthContextType => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};
