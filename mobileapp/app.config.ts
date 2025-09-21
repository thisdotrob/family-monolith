import 'dotenv/config';
import type { ExpoConfig } from 'expo/config';

const APP_ID = 'takenlijst';

const META = {
  placeholder: {
    name: 'Placeholder',
    slug: 'placeholder',
    iosBundleId: 'com.example.placeholder',
    androidPackage: 'com.example.placeholder',
    updatesChannel: 'family-placeholder',
  },
  takenlijst: {
    name: 'Family Takenlijst',
    slug: 'takenlijst',
    iosBundleId: 'com.example.takenlijst',
    androidPackage: 'com.example.takenlijst',
    updatesChannel: 'family-takenlijst',
  },
} as const;

const m = META[APP_ID as keyof typeof META];

export default ({ config }: { config: ExpoConfig }): ExpoConfig => ({
  ...config,
  name: m.name,
  slug: m.slug,
  scheme: m.slug,
  ios: {
    ...config.ios,
    bundleIdentifier: m.iosBundleId,
  },
  android: {
    ...config.android,
    package: m.androidPackage,
  },
  updates: {
    ...config.updates,
    enabled: true,
    checkAutomatically: 'ON_LOAD',
  },
  extra: {
    ...config.extra,
    // Preserve any existing EAS project linkage, or allow override via env
    eas: {
      ...(config.extra as any)?.eas,
      projectId: process.env.EAS_PROJECT_ID || (config.extra as any)?.eas?.projectId,
    },
    APP_ID,
    UPDATES_CHANNEL: m.updatesChannel,
  },
});
