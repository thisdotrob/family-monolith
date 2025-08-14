import React, { createContext, useState, useContext, ReactNode, useEffect } from 'react';
import AsyncStorage from '@react-native-async-storage/async-storage';

interface AuthContextType {
  token: string | null;
  isLoading: boolean; // To handle async storage loading
  saveTokens: (newToken: string, newRefreshToken: string) => Promise<void>;
  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | null>(null);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [token, setToken] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const loadToken = async () => {
      try {
        const storedToken = await AsyncStorage.getItem('token');
        setToken(storedToken);
      } catch (e) {
        console.error('Failed to load token from storage', e);
      } finally {
        setIsLoading(false);
      }
    };

    loadToken();
  }, []);

  const saveTokens = async (newToken: string, newRefreshToken: string) => {
    try {
      await AsyncStorage.setItem('token', newToken);
      await AsyncStorage.setItem('refreshToken', newRefreshToken);
      setToken(newToken);
    } catch (e) {
      console.error('Failed to save tokens to storage', e);
    }
  };

  const logout = async () => {
    try {
      await AsyncStorage.removeItem('token');
      await AsyncStorage.removeItem('refreshToken');
      setToken(null);
    } catch (e) {
      console.error('Failed to remove tokens from storage', e);
    }
  };

  const authContextValue = {
    token,
    isLoading,
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
