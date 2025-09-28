import { useEffect, useState } from 'react';
import { getIanaTimezone } from '@shared/time';

/**
 * useTimezone (mobile)
 *
 * Returns the best-known IANA timezone string for this device.
 * - Uses Expo Localization when available (at runtime)
 * - Falls back to Intl API via shared getIanaTimezone()
 */
export function useTimezone() {
  const [timezone, setTimezone] = useState<string>(() => getIanaTimezone());

  useEffect(() => {
    let mounted = true;

    (async () => {
      try {
        // Dynamically import to avoid build-time dependency when not available
        // @ts-ignore - type-only during dev; real app includes expo-localization types
        const mod: any = await import('expo-localization');
        // Prefer modern API if available
        let tz: string | undefined;
        if (mod?.getLocales) {
          const locales = mod.getLocales();
          if (Array.isArray(locales) && locales[0] && typeof locales[0].timeZone === 'string') {
            tz = locales[0].timeZone;
          }
        }
        if (!tz && typeof mod?.timeZone === 'string') {
          tz = mod.timeZone;
        }
        if (mounted && tz && tz.length > 0) {
          setTimezone(tz);
        }
      } catch (_e) {
        // ignore; fallback already set
      }
    })();

    return () => {
      mounted = false;
    };
  }, []);

  return timezone;
}
