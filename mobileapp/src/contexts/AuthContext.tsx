import React, { createContext, useState, useContext, useEffect, ReactNode } from 'react';
import AsyncStorage from '@react-native-async-storage/async-storage';

interface AuthContextType {
  token: string | null;
  isRefreshingToken: boolean;
  saveTokens: (newToken: string, newRefreshToken: string) => Promise<void>;
  logout: () => Promise<void>;
  setIsRefreshingToken: (isRefreshing: boolean) => void;
}

const AuthContext = createContext<AuthContextType | null>(null);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [token, setToken] = useState<string | null>(null);
  const [isLoadingToken, setIsLoadingToken] = useState(true);

  useEffect(() => {
    AsyncStorage.getItem('token')
      .then(setToken)
      .catch((e) => console.error('Failed to load token from storage', e))
      .finally(() => setIsLoadingToken(false));
  }, []);

  const [isRefreshingToken, setIsRefreshingToken] = useState<boolean>(false);

  const saveTokens = async (newToken: string, newRefreshToken: string) => {
    setToken(newToken);
    try {
      await AsyncStorage.setItem('token', newToken);
      await AsyncStorage.setItem('refreshToken', newRefreshToken);
    } catch (e) {
      console.error('Failed to save tokens to AsyncStorage', e);
    }
  };

  const logout = async () => {
    setToken(null);
    try {
      await AsyncStorage.removeItem('token');
      await AsyncStorage.removeItem('refreshToken');
    } catch (e) {
      console.error('Failed to remove tokens from AsyncStorage', e);
    }
  };

  const authContextValue = {
    token,
    isLoadingToken,
    isRefreshingToken,
    setIsRefreshingToken,
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
