/**
 * Timezone helpers for clients (web and mobile).
 *
 * - getIanaTimezone(): Best-effort IANA timezone string using the JS Intl API.
 *   Works on web and modern React Native runtimes.
 */

/**
 * Returns the device/user IANA timezone string, e.g. "Europe/Amsterdam".
 *
 * Uses Intl API which is widely available. If unavailable or missing, defaults to 'UTC'.
 */
export function getIanaTimezone(): string {
  try {
    const tz = Intl.DateTimeFormat().resolvedOptions().timeZone;
    if (typeof tz === 'string' && tz.length > 0) return tz;
  } catch (e) {
    // ignore and fall back
  }
  return 'UTC';
}

/**
 * Merge helper to attach timezone variable to GraphQL operations that require it.
 *
 * If a timezone is provided, it is used. Otherwise, if the variables already contain a
 * `timezone` property, it is preserved. If neither is present, `getIanaTimezone()` is used.
 */
export function withTimezone<T extends Record<string, any>>(
  variables: T = {} as T,
  timezone?: string,
): T & { timezone: string } {
  const tz = timezone ?? (variables as any).timezone ?? getIanaTimezone();
  return { ...variables, timezone: tz } as T & { timezone: string };
}
