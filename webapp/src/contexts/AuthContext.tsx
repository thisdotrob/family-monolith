import React, { createContext, useState, useContext, ReactNode } from 'react';

interface AuthContextType {
  token: string | null;
  isRefreshing: boolean; // Add this
  saveTokens: (newToken: string, newRefreshToken: string) => void;
  logout: () => void;
  setIsRefreshing: (isRefreshing: boolean) => void; // Add this
}

const AuthContext = createContext<AuthContextType | null>(null);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [token, setToken] = useState<string | null>(() => localStorage.getItem('token'));
  const [isRefreshing, setIsRefreshing] = useState(false); // Add this

  const saveTokens = (newToken: string, newRefreshToken: string) => {
    localStorage.setItem('token', newToken);
    localStorage.setItem('refreshToken', newRefreshToken);
    setToken(newToken);
    // We will clear the cache properly in the apollo.ts file now
  };

  const logout = () => {
    localStorage.removeItem('token');
    localStorage.removeItem('refreshToken');
    setToken(null);
    // We will clear the cache properly in the apollo.ts file now
  };

  const authContextValue = {
    token,
    isRefreshing, // Add this
    saveTokens,
    logout,
    setIsRefreshing, // Add this
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
