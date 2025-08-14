import React from 'react';
import { useAuth } from '../contexts/AuthContext';

const LoadingOverlay: React.FC = () => {
  const { isRefreshingToken } = useAuth();

  if (!isRefreshingToken) {
    return null;
  }

  return (
    <div
      data-testid="loading-overlay"
      className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
    >
      <div className="rounded-lg bg-white p-8 text-center">
        <p className="text-lg font-semibold">Refreshing JWT, hold tight...</p>
      </div>
    </div>
  );
};

export default LoadingOverlay;
