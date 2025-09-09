import Constants from 'expo-constants';
import type { ComponentType } from 'react';

// Import app modules here so Metro includes them in the bundle.
// Add new imports as you create new apps under apps/mobile/<appId>
import Placeholder from '@apps-mobile/placeholder';

const registry: Record<string, ComponentType<any>> = {
  placeholder: Placeholder,
  // Add more apps here, e.g.:
  // groceries: Groceries,
  // trips: Trips,
};

function getAppId(): string {
  // Prefer Expo config extra (set via app.config.ts), else env, else default
  const fromExtra = (Constants.expoConfig?.extra as any)?.APP_ID as string | undefined;
  const fromEnv = (process.env.APP_ID as string | undefined) || (global as any).APP_ID;
  return fromExtra || fromEnv || 'placeholder';
}

export default function selectMobileApp(): ComponentType<any> {
  const appId = getAppId();
  return registry[appId] || registry.placeholder;
}
