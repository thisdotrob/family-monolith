import { useState, useEffect } from 'react';
import { getDeviceTimezone, getDeviceTimezonSync } from './timezone';

/**
 * Hook to get the device's timezone
 * Returns the timezone immediately with a sync fallback, then updates with async result
 * 
 * @example
 * ```tsx
 * function MyComponent() {
 *   const timezone = useTimezone();
 *   
 *   // Use timezone in GraphQL queries that require it
 *   const { data } = useQuery(TASKS_QUERY, {
 *     variables: { 
 *       projectId: "123", 
 *       timezone 
 *     }
 *   });
 *   
 *   return <div>Current timezone: {timezone}</div>;
 * }
 * ```
 */
export const useTimezone = (): string => {
  // Initialize with sync timezone to avoid delays
  const [timezone, setTimezone] = useState(() => getDeviceTimezonSync());

  useEffect(() => {
    // Async update to get more accurate timezone (especially on mobile)
    getDeviceTimezone().then((asyncTimezone) => {
      if (asyncTimezone !== timezone) {
        setTimezone(asyncTimezone);
      }
    });
  }, [timezone]);

  return timezone;
};

/**
 * Helper to attach timezone variable to GraphQL operation variables
 * Useful for queries/mutations that require timezone for server-side derivations
 * 
 * @param variables - Existing GraphQL variables
 * @param timezone - IANA timezone string
 * @returns Variables object with timezone added
 * 
 * @example
 * ```tsx
 * const timezone = useTimezone();
 * const { data } = useQuery(TASKS_QUERY, {
 *   variables: withTimezone({ projectId: "123" }, timezone)
 * });
 * ```
 */
export const withTimezone = <T extends Record<string, any>>(
  variables: T,
  timezone: string,
): T & { timezone: string } => ({
  ...variables,
  timezone,
});