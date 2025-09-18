# Ticket 013: Play Activities Frontend Implementation

## Overview
Implement the frontend for play activity tracking, following the established pattern from previous activity types.

## Acceptance Criteria
- [ ] Play activity entry form with all required fields
- [ ] GraphQL operations extended for play activities
- [ ] Form submission successfully creates play activities
- [ ] Activity list displays recent play activities
- [ ] Loading states and error handling implemented
- [ ] Navigation from home screen to play screen working

## Implementation Steps

### 1. Extend GraphQL Operations
Add to existing `shared/graphql/champ-tracker.ts`:
```typescript
export const CREATE_PLAY_ACTIVITY = gql`
  mutation CreatePlayActivity($input: PlayActivityInput!) {
    createPlayActivity(input: $input) {
      id
      startTime
      durationMinutes
      playType
      participants
      location
      notes
      createdAt
    }
  }
`;

export const GET_PLAY_ACTIVITIES = gql`
  query GetPlayActivities($limit: Int, $offset: Int) {
    champTracker {
      playActivities(limit: $limit, offset: $offset) {
        id
        userId
        startTime
        durationMinutes
        playType
        participants
        location
        notes
        createdAt
      }
    }
  }
`;
```

### 2. Implement Form Component
Update `apps/mobile/champs-tracker/screens/PlayScreen.tsx`:
```typescript
export default function PlayScreen() {
  // Form state management for play activity fields
  // GraphQL mutation hook
  // Form submission handler
  // UI with start_time, duration_minutes, play_type, participants, location, notes fields
  // Submit button and loading states
}
```

### 3. Create Activity List Component
Create `apps/mobile/champs-tracker/components/PlayActivityList.tsx`:
```typescript
export default function PlayActivityList() {
  // GraphQL query hook for play activities
  // List rendering with play activity info
  // Loading and error states
  // Pull-to-refresh functionality
}
```

### 4. Update Home Screen
Update `apps/mobile/champs-tracker/screens/HomeScreen.tsx`:
- Add play activity button with 🎾 icon
- Implement navigation to PlayScreen
- Ensure consistent styling with other activity buttons

### 5. Wire Everything Together
- Import and use new play activity GraphQL operations
- Connect form submission to backend
- Display success/error messages
- Ensure proper navigation flow

## Form Fields
- **Start Time**: Date/time picker (default to current time)
- **Duration**: Number input in minutes (required)
- **Play Type**: Text input (required) - e.g. "laser pointer", "feather toy", "catnip mouse", "solo play"
- **Participants**: Text input (required) - e.g. "Champagne solo", "Champagne + John", "Champagne + family"
- **Location**: Text input (optional) - e.g. "living room", "bedroom", "garden"
- **Notes**: Multi-line text input (optional) - e.g. "very energetic", "lost interest quickly"

## Files to Create/Modify
- `shared/graphql/champ-tracker.ts` (extend existing)
- `apps/mobile/champs-tracker/screens/PlayScreen.tsx`
- `apps/mobile/champs-tracker/components/PlayActivityList.tsx`
- `apps/mobile/champs-tracker/screens/HomeScreen.tsx`

## Testing Notes
- Verify form submits correctly and creates backend records
- Test all form fields work as expected
- Confirm activity list displays created play activities
- Test navigation flow from home → play → back
- Verify loading states and error handling
- Test duration input accepts only positive numbers
- Test participants field captures who was involved

## Dependencies
- Backend implementation from ticket 012
- Completed tickets 003, 005, 007, 009, 011 (established frontend pattern)
- Apollo Client setup
- React Navigation

## Estimated Effort
Small - Following established pattern from previous activity frontends.