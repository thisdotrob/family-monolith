# Ticket 007: Outdoor Activities Frontend Implementation

## Overview
Implement the frontend for outdoor activity tracking, following the established pattern from previous activity types.

## Acceptance Criteria
- [ ] Outdoor activity entry form with all required fields
- [ ] GraphQL operations extended for outdoor activities
- [ ] Form submission successfully creates outdoor activities
- [ ] Activity list displays recent outdoor activities
- [ ] Loading states and error handling implemented
- [ ] Navigation from home screen to outdoor screen working

## Implementation Steps

### 1. Extend GraphQL Operations
Add to existing `shared/graphql/champ-tracker.ts`:
```typescript
export const CREATE_OUTDOOR_ACTIVITY = gql`
  mutation CreateOutdoorActivity($input: OutdoorActivityInput!) {
    createOutdoorActivity(input: $input) {
      id
      startTimestamp
      durationMinutes
      activityType
      behavior
      location
      createdAt
    }
  }
`;

export const GET_OUTDOOR_ACTIVITIES = gql`
  query GetOutdoorActivities($limit: Int, $offset: Int) {
    champTracker {
      outdoorActivities(limit: $limit, offset: $offset) {
        id
        userId
        startTimestamp
        durationMinutes
        activityType
        behavior
        location
        createdAt
      }
    }
  }
`;
```

### 2. Implement Form Component
Update `apps/mobile/champs-tracker/screens/OutdoorScreen.tsx`:
```typescript
export default function OutdoorScreen() {
  // Form state management for outdoor fields
  // GraphQL mutation hook
  // Form submission handler
  // UI with start_timestamp, duration_minutes, activity_type, behavior, location fields
  // Submit button and loading states
}
```

### 3. Create Activity List Component
Create `apps/mobile/champs-tracker/components/OutdoorActivityList.tsx`:
```typescript
export default function OutdoorActivityList() {
  // GraphQL query hook for outdoor activities
  // List rendering with outdoor activity info
  // Loading and error states
  // Pull-to-refresh functionality
}
```

### 4. Update Home Screen
Update `apps/mobile/champs-tracker/screens/HomeScreen.tsx`:
- Add outdoor activity button with 🌳 icon
- Implement navigation to OutdoorScreen
- Ensure consistent styling with other activity buttons

### 5. Wire Everything Together
- Import and use new outdoor GraphQL operations
- Connect form submission to backend
- Display success/error messages
- Ensure proper navigation flow

## Form Fields
- **Start Timestamp**: Date/time picker (default to current time)
- **Duration**: Number input in minutes (required)
- **Activity Type**: Text input or picker (required) - e.g. "pram", "walking", "free roam"
- **Behavior**: Multi-line text input (optional) - e.g. "exploring", "hunting", "sleeping"
- **Location**: Text input (optional) - e.g. "backyard", "front porch", "garden"

## Files to Create/Modify
- `shared/graphql/champ-tracker.ts` (extend existing)
- `apps/mobile/champs-tracker/screens/OutdoorScreen.tsx`
- `apps/mobile/champs-tracker/components/OutdoorActivityList.tsx`
- `apps/mobile/champs-tracker/screens/HomeScreen.tsx`

## Testing Notes
- Verify form submits correctly and creates backend records
- Test all form fields work as expected
- Confirm activity list displays created outdoor activities
- Test navigation flow from home → outdoor → back
- Verify loading states and error handling
- Test duration input accepts only positive numbers

## Dependencies
- Backend implementation from ticket 006
- Completed tickets 003 and 005 (established frontend pattern)
- Apollo Client setup
- React Navigation

## Estimated Effort
Small - Following established pattern from previous activity frontends.