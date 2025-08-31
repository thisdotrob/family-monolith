/// <reference types="vitest" />
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import path from 'node:path'

const rootDir = __dirname

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      '@shared': path.resolve(rootDir, '../shared'),
      '@apollo/client': path.resolve(rootDir, 'node_modules/@apollo/client'),
      'graphql': path.resolve(rootDir, 'node_modules/graphql'),
    },
    preserveSymlinks: true,
  },
  server: {
    fs: {
      // Allow serving files from the project root's shared directory
      allow: [path.resolve(rootDir, '..')],
    },
  },
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './vitest.setup.ts',
  },
})

