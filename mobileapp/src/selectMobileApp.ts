import type { ComponentType } from 'react';
import ChampsTracker from '@apps-mobile/champs-tracker';

export default function selectMobileApp(): ComponentType<any> {
  // For now, default to champs-tracker
  // Later this could be environment-driven or user-selectable
  return ChampsTracker;
}
