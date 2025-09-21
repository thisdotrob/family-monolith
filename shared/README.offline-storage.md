# Shared: Offline hook and storage keys

This package provides:
- `useOffline()` hook (platform-aware)
- storage helpers for last-selected project and saved view

## useOffline()

Web:
```tsx
import { useOffline } from '@shared/hooks/useOffline';

export function Status() {
  const isOffline = useOffline();
  return <div>{isOffline ? 'Offline' : 'Online'}</div>;
}
```

Mobile (Expo/React Native): ensure `@react-native-community/netinfo` is installed.
```tsx
import { useOffline } from '@shared/hooks/useOffline';
import { Text } from 'react-native';

export function Status() {
  const isOffline = useOffline();
  return <Text>{isOffline ? 'Offline' : 'Online'}</Text>;
}
```

## Storage helpers

```ts
import type LocalStorage from 'webapp/src/LocalStorage'; // or mobileapp/src/LocalStorage
import {
  STORAGE_KEYS,
  setLastSelectedProjectId,
  getLastSelectedProjectId,
  setLastSavedViewId,
  getLastSavedViewId,
} from '@shared/storage/keys';

async function demo(storage: typeof LocalStorage) {
  await setLastSelectedProjectId(storage, 'project-123');
  const projectId = await getLastSelectedProjectId(storage);

  await setLastSavedViewId(storage, 'inbox');
  const savedViewId = await getLastSavedViewId(storage);
}
```

Notes:
- Do not store auth tokens with these helpers. Auth tokens are managed solely by `AuthContext` using its own keys.
- Keys are namespaced under the `todo:` prefix.
