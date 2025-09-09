/// <reference types="vitest" />
import { defineConfig, loadEnv } from 'vite';
import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';
import path from 'node:path';

const rootDir = __dirname;

// https://vite.dev/config/
export default defineConfig(({ mode }) => {
  // Load env variables (e.g., VITE_APP_ID)
  const env = loadEnv(mode, process.cwd(), '');
  const appId = env.VITE_APP_ID;

  if (!appId) {
    throw new Error('VITE_APP_ID not set');
  }

  function toTitleCase(id: string): string {
    const parts = id
      .replace(/([a-z0-9])([A-Z])/g, '$1 $2')
      .split(/[-_\s]+/)
      .filter(Boolean);
    return parts.map((p) => p.charAt(0).toUpperCase() + p.slice(1).toLowerCase()).join(' ');
  }

  const appTitle = toTitleCase(appId);

  if (!appId) {
    throw new Error('VITE_APP_ID not set');
  }

  return {
    plugins: [
      react(),
      tailwindcss(),
      {
        name: 'html-title-inject',
        transformIndexHtml(html) {
          return html.replace(/%APP_TITLE%/g, appTitle);
        },
      },
    ],
    resolve: {
      alias: {
        '@shared': path.resolve(rootDir, '../shared'),
        '@apps-web': path.resolve(rootDir, '../apps/web'),
        '@apollo/client': path.resolve(rootDir, 'node_modules/@apollo/client'),
        graphql: path.resolve(rootDir, 'node_modules/graphql'),
      },
      preserveSymlinks: true,
    },
    server: {
      fs: {
        // Allow serving files from the project root's shared directory
        allow: [path.resolve(rootDir, '..')],
      },
    },
    base: `/${appId}/`,
    build: {
      // Helpful hint for multi-app deployments: put artifacts under dist/<appId>/
      outDir: `dist/${appId}`,
    },
    test: {
      globals: true,
      environment: 'jsdom',
      setupFiles: './vitest.setup.ts',
    },
  };
});
