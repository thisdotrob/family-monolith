# Ticket 001: Project Foundation and App Structure

## Overview
Set up the basic project structure for the champs-tracker mobile app, including app selection integration, basic navigation, and authentication setup.

## Acceptance Criteria
- [ ] New app directory created at `apps/mobile/champs-tracker/`
- [ ] App properly integrated into mobile app selection system
- [ ] Basic navigation structure in place with placeholder screens
- [ ] Authentication integration working (login/logout)
- [ ] App displays "Champs Tracker" branding
- [ ] Placeholder app icon configured (to be replaced with Champ's photo)

## Implementation Steps

### 1. Create App Directory Structure
```
apps/mobile/champs-tracker/
├── index.ts
├── HomePage.tsx
├── navigation/
│   ├── AppNavigator.tsx
│   └── types.ts
├── screens/
│   ├── HomeScreen.tsx
│   ├── BathroomScreen.tsx
│   ├── EatingScreen.tsx
│   ├── OutdoorScreen.tsx
│   ├── VetScreen.tsx
│   ├── MedicationScreen.tsx
│   ├── PlayScreen.tsx
│   └── HighlightsScreen.tsx
└── components/
    └── ActivityButton.tsx
```

### 2. Update App Selection
- Modify `mobileapp/src/selectMobileApp.ts` to include `champs-tracker` option
- Update the selection logic to load the new app

### 3. Create Basic Navigation
- Set up React Navigation with tabs or stack navigator
- Create placeholder screens for all activity types
- Implement basic navigation between home and activity screens

### 4. Authentication Integration
- Import and use existing `AuthContext` from `@shared/contexts/AuthContext`
- Add login/logout capability
- Ensure app respects authentication state

### 5. App Icon and Branding
- Create placeholder app icon (note: replace with Champ's photo later)
- Set app display name to "Champs Tracker"
- Configure splash screen with branding

## Files to Create/Modify
- `apps/mobile/champs-tracker/` (entire directory)
- `mobileapp/src/selectMobileApp.ts`
- `mobileapp/app.config.ts` (if needed for app icon)

## Testing Notes
- Verify app loads without errors
- Test navigation between screens
- Confirm authentication flow works
- Check app icon appears correctly

## Dependencies
- React Navigation
- Existing authentication system
- Apollo Client setup

## Estimated Effort
Medium - Foundation setup with multiple moving parts but well-defined scope.