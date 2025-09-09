import 'dotenv/config';
import type { ExpoConfig } from 'expo/config';

const APP_ID = 'placeholder';

const META = {
  placeholder: {
    name: 'Placeholder',
    slug: 'placeholder',
    iosBundleId: 'com.example.placeholder',
    androidPackage: 'com.example.placeholder',
    updatesChannel: 'family-placeholder',
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
    APP_ID,
    UPDATES_CHANNEL: m.updatesChannel,
  },
});
