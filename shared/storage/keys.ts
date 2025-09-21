import type { StorageAdapter } from '../contexts/AuthContext';

// Namespaced storage keys for app state (never store auth tokens here)
export const STORAGE_KEYS = {
  LAST_PROJECT_ID: 'todo:lastProjectId',
  LAST_SAVED_VIEW_ID: 'todo:lastSavedViewId',
} as const;

// Project selection helpers
export async function setLastSelectedProjectId(storage: StorageAdapter, projectId: string): Promise<void> {
  await Promise.resolve(storage.setItem(STORAGE_KEYS.LAST_PROJECT_ID, projectId));
}

export async function getLastSelectedProjectId(storage: StorageAdapter): Promise<string | null> {
  return Promise.resolve(storage.getItem(STORAGE_KEYS.LAST_PROJECT_ID));
}

export async function removeLastSelectedProjectId(storage: StorageAdapter): Promise<void> {
  await Promise.resolve(storage.removeItem(STORAGE_KEYS.LAST_PROJECT_ID));
}

// Saved view helpers
export async function setLastSavedViewId(storage: StorageAdapter, savedViewId: string): Promise<void> {
  await Promise.resolve(storage.setItem(STORAGE_KEYS.LAST_SAVED_VIEW_ID, savedViewId));
}

export async function getLastSavedViewId(storage: StorageAdapter): Promise<string | null> {
  return Promise.resolve(storage.getItem(STORAGE_KEYS.LAST_SAVED_VIEW_ID));
}

export async function removeLastSavedViewId(storage: StorageAdapter): Promise<void> {
  await Promise.resolve(storage.removeItem(STORAGE_KEYS.LAST_SAVED_VIEW_ID));
}
