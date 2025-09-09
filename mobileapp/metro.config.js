// Learn more https://docs.expo.dev/guides/customizing-metro
const path = require('path');
const { getDefaultConfig } = require('expo/metro-config');

/** @type {import('expo/metro-config').MetroConfig} */
const config = getDefaultConfig(__dirname);

// Ensure shared code resolves React and other deps from mobileapp's node_modules
config.resolver = config.resolver || {};
config.resolver.extraNodeModules = {
  react: path.resolve(__dirname, 'node_modules/react'),
  '@shared': path.resolve(__dirname, '..', 'shared'),
  '@apps-mobile': path.resolve(__dirname, '..', 'apps', 'mobile'),
};

// Provide resolver aliases for monorepo-style absolute imports
config.resolver.alias = {
  '@shared': path.resolve(__dirname, '..', 'shared'),
  '@apps-mobile': path.resolve(__dirname, '..', 'apps', 'mobile'),
};

// Exclude test files from the bundle
config.resolver.blockList = [/(.*\.test\.tsx?)$/];

// Allow importing shared code and apps from the repository root
config.watchFolders = [
  path.resolve(__dirname, '..', 'shared'),
  path.resolve(__dirname, '..', 'apps', 'mobile'),
];

module.exports = config;
