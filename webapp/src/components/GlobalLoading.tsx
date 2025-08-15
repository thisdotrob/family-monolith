import React from 'react';
import { useAuth } from '../contexts/AuthContext';

const GlobalLoading = () => {
  const { isRefreshingToken } = useAuth();

  if (!isRefreshingToken) {
    return null;
  }

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center">
      <div className="bg-white p-6 rounded-lg shadow-xl">
        <p className="text-lg font-semibold">Refreshing session...</p>
      </div>
    </div>
  );
};

export default GlobalLoading;
