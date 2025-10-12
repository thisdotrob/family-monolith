// import type { ComponentType } from 'react';
import Constants from 'expo-constants';
import Placeholder from '@apps-mobile/placeholder';
import Takenlijst from '@apps-mobile/takenlijst';

const APPS: Record<string, any> = {
  placeholder: Placeholder,
  takenlijst: Takenlijst,
};

export default function selectMobileApp(): any {
  // simplified return type to avoid cross-package react types
  const appId =
    // SDK 49+ development env
    (Constants as any)?.expoConfig?.extra?.APP_ID ??
    // fallback in production / classic envs
    (Constants as any)?.manifestExtra?.APP_ID ??
    'placeholder';

  return APPS[appId] ?? Placeholder;
}
