// Learn more https://docs.expo.dev/guides/customizing-metro
const path = require('path');
const { getDefaultConfig } = require('expo/metro-config');

/** @type {import('expo/metro-config').MetroConfig} */
const config = getDefaultConfig(__dirname);

// Ensure shared code resolves React and other deps from mobileapp's node_modules
config.resolver = config.resolver || {};
config.resolver.extraNodeModules = {
  react: path.resolve(__dirname, 'node_modules/react'),
};

// Exclude test files from the bundle
config.resolver.blockList = [/(.*\.test\.tsx?)$/];

// Allow importing shared code from the repository root
config.watchFolders = [path.resolve(__dirname, '..', 'shared')];

module.exports = config;
