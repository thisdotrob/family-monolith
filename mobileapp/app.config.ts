import 'dotenv/config';
import type { ExpoConfig } from 'expo/config';

const APP_ID = process.env.APP_ID || 'placeholder';

const META = {
  placeholder: {
    name: 'Placeholder',
    slug: 'placeholder',
    iosBundleId: 'com.example.placeholder',
    androidPackage: 'com.example.placeholder',
  },
  // Add additional apps here:
  // groceries: {
  //   name: 'Groceries',
  //   slug: 'groceries',
  //   iosBundleId: 'com.example.groceries',
  //   androidPackage: 'com.example.groceries',
  // },
} as const;

const m = META[APP_ID as keyof typeof META] ?? META.placeholder;

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
  extra: {
    ...config.extra,
    APP_ID,
  },
});
