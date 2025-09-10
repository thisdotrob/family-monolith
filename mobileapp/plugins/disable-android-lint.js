const { withAppBuildGradle } = require('@expo/config-plugins');

// Disable lint failures on release builds to avoid flaky lint crashes in dependencies
module.exports = function disableAndroidLint(config) {
  return withAppBuildGradle(config, (config) => {
    try {
      let contents = config.modResults.contents;
      const isGroovy = config.modResults.language === 'groovy';

      if (isGroovy) {
        if (!contents.includes('lintOptions')) {
          contents = contents.replace(
            /android\s*{/,
            (m) =>
              `${m}\n    lintOptions {\n        checkReleaseBuilds false\n        abortOnError false\n    }\n`,
          );
        }
      } else {
        // Kotlin DSL (build.gradle.kts)
        if (!contents.includes('lint {')) {
          contents = contents.replace(
            /android\s*{/,
            (m) =>
              `${m}\n    lint {\n        checkReleaseBuilds = false\n        abortOnError = false\n    }\n`,
          );
        }
      }

      config.modResults.contents = contents;
    } catch (e) {
      console.warn('disable-android-lint plugin failed to modify app build.gradle:', e);
    }
    return config;
  });
};
