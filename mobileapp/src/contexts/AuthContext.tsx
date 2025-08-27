import React, { createContext, useState, useContext, useEffect, ReactNode } from 'react';
import AsyncStorage from '@react-native-async-storage/async-storage';

interface AuthContextType {
  isAuthenticating: boolean;
  setIsAuthenticating: (isAuthenticating: boolean) => void;
  isLoggedIn: boolean;
  saveTokens: (token: string, refreshToken: string) => Promise<void>;
  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | null>(null);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [isAuthenticating, setIsAuthenticating] = useState(true);
  const [isLoggedIn, setIsLoggedIn] = useState(true);

  useEffect(() => {
    AsyncStorage.getItem('token')
      .then((token) => setIsLoggedIn(!!token))
      .catch((e) => {
        console.error('Failed to load token from storage', e);
        setIsLoggedIn(false);
      })
      .finally(() => setIsAuthenticating(false));
  }, []);

  const saveTokens = async (newToken: string, newRefreshToken: string) => {
    try {
      await AsyncStorage.setItem('token', newToken);
      await AsyncStorage.setItem('refreshToken', newRefreshToken);
      setIsLoggedIn(true);
    } catch (e) {
      console.error('Failed to save tokens to AsyncStorage', e);
      setIsLoggedIn(false);
    } finally {
      setIsAuthenticating(false);
    }
  };

  const logout = async () => {
    try {
      await AsyncStorage.removeItem('token');
      await AsyncStorage.removeItem('refreshToken');
    } catch (e) {
      console.error('Failed to remove tokens from AsyncStorage', e);
    } finally {
      setIsLoggedIn(false);
      setIsAuthenticating(false);
    }
  };

  const authContextValue = {
    isAuthenticating,
    setIsAuthenticating,
    isLoggedIn,
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
