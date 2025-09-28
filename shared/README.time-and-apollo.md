# Shared: Timezone helper and Apollo usage

This package provides helpers to retrieve the user's IANA timezone and a small utility for attaching it to GraphQL operations. It also documents the Apollo usage pattern for unauthenticated operations.

Spec references: §§6, 8, 11, 16 in `todo-app-spec.md`.

## Timezone helpers

- `getIanaTimezone()` — Returns a best-effort IANA timezone string (e.g., `Europe/Amsterdam`). Uses the JS `Intl` API and falls back to `UTC`.
- `withTimezone(variables, tz?)` — Returns a copy of variables guaranteed to include a `timezone` property. Uses the provided `tz`, or an existing `variables.timezone`, or falls back to `getIanaTimezone()`.

### Mobile example: useTimezone hook

```tsx
import { useEffect, useState } from 'react';
import { getIanaTimezone } from '@shared/time';

export function useTimezone() {
  const [timezone, setTimezone] = useState(() => getIanaTimezone());
  useEffect(() => {
    (async () => {
      try {
        const mod: any = await import('expo-localization');
        let tz: string | undefined;
        if (mod?.getLocales) {
          const locales = mod.getLocales();
          if (Array.isArray(locales) && locales[0] && typeof locales[0].timeZone === 'string') {
            tz = locales[0].timeZone;
          }
        }
        if (!tz && typeof mod?.timeZone === 'string') tz = mod.timeZone;
        if (tz) setTimezone(tz);
      } catch {}
    })();
  }, []);
  return timezone;
}
```

## Apollo usage pattern

- For unauthenticated operations (e.g., `login`, `refreshToken`), pass `context: { unauthenticated: true }` to the Apollo call. The shared `createApolloClient` respects this and will not attach the Authorization header.
- When calling queries/mutations that require timezone-derived behavior, include the `timezone` variable. You can use `withTimezone()` to guarantee it is present.

Examples:

```ts
// Login (unauthenticated)
login({ variables: { username, password }, context: { unauthenticated: true } });

// Tasks list with timezone
client.query({
  query: TASKS_QUERY,
  variables: withTimezone({ projectId }),
});
```
