/**
 * Timezone utilities for obtaining device timezone information
 * Supports both mobile (expo-localization) and web (Intl API) environments
 */

/**
 * Get the device's IANA timezone string
 * Uses expo-localization on mobile with Intl API as fallback
 * @returns Promise resolving to IANA timezone string (e.g., "America/New_York")
 */
export const getDeviceTimezone = async (): Promise<string> => {
  try {
    // Try to use expo-localization if available (mobile environment)
    const ExpoLocalization: any = await import('expo-localization').catch(() => null);
    if (ExpoLocalization?.getLocales) {
      const locales: any[] = ExpoLocalization.getLocales();
      if (locales?.[0]?.timeZone) {
        return locales[0].timeZone as string;
      }
    }
  } catch (error) {
    // expo-localization not available or failed, fall back to Intl API
    console.warn('expo-localization not available, using Intl API fallback:', error);
  }

  // Fallback to JavaScript Intl API (works in web and React Native)
  try {
    const timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
    if (timezone) {
      return timezone;
    }
  } catch (error) {
    console.warn('Intl API timezone resolution failed:', error);
  }

  // Ultimate fallback to UTC if all else fails
  console.warn('Unable to determine device timezone, defaulting to UTC');
  return 'UTC';
};

/**
 * Get the device's IANA timezone string synchronously
 * Only uses Intl API (for cases where async is not needed)
 * @returns IANA timezone string (e.g., "America/New_York") or "UTC" as fallback
 */
export const getDeviceTimezonSync = (): string => {
  try {
    const timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
    if (timezone) {
      return timezone;
    }
  } catch (error) {
    console.warn('Intl API timezone resolution failed:', error);
  }

  // Fallback to UTC if Intl API fails
  console.warn('Unable to determine device timezone, defaulting to UTC');
  return 'UTC';
};

/**
 * Validates if a given string is a valid IANA timezone
 * @param timezone - String to validate
 * @returns boolean indicating if the timezone is valid
 */
export const isValidTimezone = (timezone: string): boolean => {
  try {
    Intl.DateTimeFormat(undefined, { timeZone: timezone });
    return true;
  } catch {
    return false;
  }
};