import { useEffect, useState } from 'react';

/**
 * Web implementation of offline detection using navigator.onLine
 * and window online/offline events.
 * Returns true when offline.
 */
export function useOffline(): boolean {
  const getInitial = () => {
    if (typeof navigator !== 'undefined' && 'onLine' in navigator) {
      return !navigator.onLine;
    }
    return false;
  };

  const [isOffline, setIsOffline] = useState<boolean>(getInitial);

  useEffect(() => {
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);

    if (typeof window !== 'undefined' && typeof window.addEventListener === 'function') {
      window.addEventListener('online', handleOnline);
      window.addEventListener('offline', handleOffline);
      // Sync initial state
      if (typeof navigator !== 'undefined' && 'onLine' in navigator) {
        setIsOffline(!navigator.onLine);
      }
      return () => {
        window.removeEventListener('online', handleOnline);
        window.removeEventListener('offline', handleOffline);
      };
    }

    return () => {};
  }, []);

  return isOffline;
}
