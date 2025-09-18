# Ticket 003: Bathroom Activities Frontend Implementation

## Overview
Implement the frontend for bathroom activity tracking, including form UI, GraphQL integration, and connection to the backend from ticket 002.

## Acceptance Criteria
- [ ] Bathroom activity entry form with all required fields
- [ ] GraphQL mutations and queries defined in shared directory
- [ ] Form submission successfully creates bathroom activities
- [ ] Activity list displays recent bathroom activities
- [ ] Loading states and error handling implemented
- [ ] Navigation from home screen to bathroom screen working

## Implementation Steps

### 1. Create GraphQL Operations
Add to `shared/graphql/champ-tracker.ts` (new file):
```typescript
import { gql } from '@apollo/client';

export const CREATE_BATHROOM_ACTIVITY = gql`
  mutation CreateBathroomActivity($input: BathroomActivityInput!) {
    createBathroomActivity(input: $input) {
      id
      timestamp
      consistency
      observations
      litterChanged
      createdAt
    }
  }
`;

export const GET_BATHROOM_ACTIVITIES = gql`
  query GetBathroomActivities($limit: Int, $offset: Int) {
    champTracker {
      bathroomActivities(limit: $limit, offset: $offset) {
        id
        userId
        timestamp
        consistency
        observations
        litterChanged
        createdAt
      }
    }
  }
`;
```

### 2. Create Form Component
Update `apps/mobile/champs-tracker/screens/BathroomScreen.tsx`:
```typescript
export default function BathroomScreen() {
  // Form state management
  // GraphQL mutation hook
  // Form submission handler
  // UI with timestamp, consistency, observations, litter_changed fields
  // Submit button and loading states
}
```

### 3. Create Activity List Component
Create `apps/mobile/champs-tracker/components/BathroomActivityList.tsx`:
```typescript
export default function BathroomActivityList() {
  // GraphQL query hook
  // List rendering with basic activity info
  // Loading and error states
  // Pull-to-refresh functionality
}
```

### 4. Update Home Screen Navigation
Update `apps/mobile/champs-tracker/screens/HomeScreen.tsx`:
- Add bathroom activity button with 💩 icon
- Implement navigation to BathroomScreen
- Style activity button grid layout

### 5. Wire Everything Together
- Import and use new GraphQL operations
- Connect form submission to backend
- Display success/error messages
- Ensure proper navigation flow

## Form Fields
- **Timestamp**: Date/time picker (default to current time)
- **Consistency**: Text input (optional)
- **Observations**: Multi-line text input (optional)
- **Litter Changed**: Boolean toggle/switch (required)

## Files to Create/Modify
- `shared/graphql/champ-tracker.ts` (new file)
- `apps/mobile/champs-tracker/screens/BathroomScreen.tsx`
- `apps/mobile/champs-tracker/components/BathroomActivityList.tsx`
- `apps/mobile/champs-tracker/screens/HomeScreen.tsx`

## Testing Notes
- Verify form submits correctly and creates backend records
- Test all form fields work as expected
- Confirm activity list displays created activities
- Test navigation flow from home → bathroom → back
- Verify loading states and error handling
- Test with both valid and invalid inputs

## Dependencies
- Backend implementation from ticket 002
- Apollo Client setup
- React Navigation
- Form handling libraries (React Hook Form or similar)

## Estimated Effort
Medium - Frontend form implementation with GraphQL integration.