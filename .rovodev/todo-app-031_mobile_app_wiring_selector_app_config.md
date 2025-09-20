# 031 — Mobile: App Wiring (Selector + app.config)

Spec refs: §16

## Summary
Wire the new `takenlijst` module into `mobileapp/src/selectMobileApp.ts` and `mobileapp/app.config.ts` to set slug/name and enable runtime selection.

## Scope
- Update selector to support `takenlijst`
- Update app.config to name/slug/bundle/app id; ensure extra.APP_ID="takenlijst"

## Acceptance Criteria
- Running the app shows the `takenlijst` module

## Dependencies
- 030

## Implementation Steps
1) Edit selector to import `@apps-mobile/takenlijst`
2) Update app.config values
