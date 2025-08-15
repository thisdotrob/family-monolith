import React, { createContext, useState, useContext, ReactNode } from 'react';

interface AuthContextType {
  token: string | null;
  isRefreshingToken: boolean;
  saveTokens: (newToken: string, newRefreshToken: string) => void;
  logout: () => void;
  setIsRefreshingToken: (isRefreshing: boolean) => void;
}

const AuthContext = createContext<AuthContextType | null>(null);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [token, setToken] = useState<string | null>(() => localStorage.getItem('token'));
  const [isRefreshingToken, setIsRefreshingToken] = useState<boolean>(false);

  const saveTokens = (newToken: string, newRefreshToken: string) => {
    localStorage.setItem('token', newToken);
    localStorage.setItem('refreshToken', newRefreshToken);
    setToken(newToken);
  };

  const logout = () => {
    localStorage.removeItem('token');
    localStorage.removeItem('refreshToken');
    setToken(null);
  };

  const authContextValue = {
    token,
    isRefreshingToken,
    saveTokens,
    logout,
    setIsRefreshingToken,
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
