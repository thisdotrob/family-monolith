import React, { createContext, useContext, useEffect, useMemo, useState } from 'react';
import type { ReactNode } from 'react';
import LocalStorage from '../LocalStorage';
import {
  getLastSavedViewId as storageGetLastSavedViewId,
  getLastSelectedProjectId as storageGetLastSelectedProjectId,
  removeLastSavedViewId as storageRemoveLastSavedViewId,
  removeLastSelectedProjectId as storageRemoveLastSelectedProjectId,
  setLastSavedViewId as storageSetLastSavedViewId,
  setLastSelectedProjectId as storageSetLastSelectedProjectId,
} from '@shared/storage/keys';

export type LastSelectionState = {
  lastProjectId: string | null;
  lastSavedViewId: string | null;
  isRestoring: boolean;
};

export type LastSelectionContextType = LastSelectionState & {
  setLastProjectId: (projectId: string | null) => Promise<void>;
  setLastSavedViewId: (savedViewId: string | null) => Promise<void>;
  clear: () => Promise<void>;
};

const LastSelectionContext = createContext<LastSelectionContextType | null>(null);

export function LastSelectionProvider({ children }: { children: ReactNode }) {
  const [state, setState] = useState<LastSelectionState>({ lastProjectId: null, lastSavedViewId: null, isRestoring: true });

  useEffect(() => {
    let mounted = true;

    (async () => {
      try {
        const [projectId, savedViewId] = await Promise.all([
          storageGetLastSelectedProjectId(LocalStorage),
          storageGetLastSavedViewId(LocalStorage),
        ]);
        if (!mounted) return;
        setState({ lastProjectId: projectId, lastSavedViewId: savedViewId, isRestoring: false });
      } catch (e) {
        if (__DEV__) console.warn('Failed to restore last selection', e);
        if (mounted) setState((s) => ({ ...s, isRestoring: false }));
      }
    })();

    return () => { mounted = false; };
  }, []);

  const api = useMemo<LastSelectionContextType>(() => ({
    ...state,
    setLastProjectId: async (projectId: string | null) => {
      try {
        if (projectId === null) {
          await storageRemoveLastSelectedProjectId(LocalStorage);
        } else {
          await storageSetLastSelectedProjectId(LocalStorage, projectId);
        }
        setState((prev) => ({ ...prev, lastProjectId: projectId }));
      } catch (e) {
        if (__DEV__) console.warn('Failed to set last project id', e);
      }
    },
    setLastSavedViewId: async (savedViewId: string | null) => {
      try {
        if (savedViewId === null) {
          await storageRemoveLastSavedViewId(LocalStorage);
        } else {
          await storageSetLastSavedViewId(LocalStorage, savedViewId);
        }
        setState((prev) => ({ ...prev, lastSavedViewId: savedViewId }));
      } catch (e) {
        if (__DEV__) console.warn('Failed to set last saved view id', e);
      }
    },
    clear: async () => {
      try {
        await Promise.all([
          storageRemoveLastSelectedProjectId(LocalStorage),
          storageRemoveLastSavedViewId(LocalStorage),
        ]);
        setState({ lastProjectId: null, lastSavedViewId: null, isRestoring: false });
      } catch (e) {
        if (__DEV__) console.warn('Failed to clear last selection', e);
      }
    },
  }), [state]);

  return (
    <LastSelectionContext.Provider value={api}>
      {children}
    </LastSelectionContext.Provider>
  );
}

export function useLastSelection(): LastSelectionContextType {
  const ctx = useContext(LastSelectionContext);
  if (!ctx) {
    throw new Error('useLastSelection must be used within a LastSelectionProvider');
  }
  return ctx;
}
