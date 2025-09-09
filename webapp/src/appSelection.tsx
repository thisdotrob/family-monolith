import type { ComponentType } from 'react';

export const getSelectedWebApp = (): ComponentType => {
  const appId = import.meta.env.VITE_APP_ID;

  if (!appId) {
    throw new Error('VITE_APP_ID not set');
  }

  // Eagerly import all app entrypoints so Vite can statically include them
  const modules = import.meta.glob('@apps-web/*/index.ts', { eager: true }) as Record<string, any>;

  // Try to find a module whose key ends with the expected app path
  const match = Object.entries(modules).find(([key]) => key.endsWith(`/${appId}/index.ts`));

  if (match && match[1]?.default) {
    return match[1].default as ComponentType;
  }

  throw new Error(`No web app module found for VITE_APP_ID="${appId}".`);
};
