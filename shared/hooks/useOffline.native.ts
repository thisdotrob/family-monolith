import { useEffect, useRef, useState } from 'react';
import NetInfo, { type NetInfoState } from '@react-native-community/netinfo';

/**
 * React Native implementation using @react-native-community/netinfo
 * Returns true when offline.
 */
export function useOffline(): boolean {
  const [isOffline, setIsOffline] = useState<boolean>(false);
  const unsubRef = useRef<null | (() => void)>(null);

  useEffect(() => {
    let mounted = true;

    (async () => {
      try {
        const initial = await NetInfo.fetch();
        if (!mounted) return;
        const isConnected = initial.isInternetReachable ?? initial.isConnected ?? true;
        setIsOffline(!isConnected);
      } catch {
        if (mounted) setIsOffline(false);
      }
    })();

    const unsubscribe = NetInfo.addEventListener((state: NetInfoState) => {
      const reachable = state.isInternetReachable ?? state.isConnected ?? true;
      setIsOffline(!reachable);
    });
    unsubRef.current = unsubscribe;

    return () => {
      mounted = false;
      if (unsubRef.current) {
        try { unsubRef.current(); } catch { /* noop */ }
      }
    };
  }, []);

  return isOffline;
}
