# Ticket 005: Eating Activities Frontend Implementation

## Overview
Implement the frontend for eating activity tracking, following the same pattern established in ticket 003 for bathroom activities.

## Acceptance Criteria
- [ ] Eating activity entry form with all required fields
- [ ] GraphQL operations extended for eating activities
- [ ] Form submission successfully creates eating activities
- [ ] Activity list displays recent eating activities
- [ ] Loading states and error handling implemented
- [ ] Navigation from home screen to eating screen working

## Implementation Steps

### 1. Extend GraphQL Operations
Add to existing `shared/graphql/champ-tracker.ts`:
```typescript
export const CREATE_EATING_ACTIVITY = gql`
  mutation CreateEatingActivity($input: EatingActivityInput!) {
    createEatingActivity(input: $input) {
      id
      timestamp
      quantityEaten
      leftoversThrowAway
      foodType
      createdAt
    }
  }
`;

export const GET_EATING_ACTIVITIES = gql`
  query GetEatingActivities($limit: Int, $offset: Int) {
    champTracker {
      eatingActivities(limit: $limit, offset: $offset) {
        id
        userId
        timestamp
        quantityEaten
        leftoversThrowAway
        foodType
        createdAt
      }
    }
  }
`;
```

### 2. Implement Form Component
Update `apps/mobile/champs-tracker/screens/EatingScreen.tsx`:
```typescript
export default function EatingScreen() {
  // Form state management for eating fields
  // GraphQL mutation hook
  // Form submission handler
  // UI with timestamp, quantity_eaten, leftovers_thrown_away, food_type fields
  // Submit button and loading states
}
```

### 3. Create Activity List Component
Create `apps/mobile/champs-tracker/components/EatingActivityList.tsx`:
```typescript
export default function EatingActivityList() {
  // GraphQL query hook for eating activities
  // List rendering with eating activity info
  // Loading and error states
  // Pull-to-refresh functionality
}
```

### 4. Update Home Screen
Update `apps/mobile/champs-tracker/screens/HomeScreen.tsx`:
- Add eating activity button with 🍽️ icon
- Implement navigation to EatingScreen
- Ensure consistent styling with bathroom button

### 5. Wire Everything Together
- Import and use new eating GraphQL operations
- Connect form submission to backend
- Display success/error messages
- Ensure proper navigation flow

## Form Fields
- **Timestamp**: Date/time picker (default to current time)
- **Quantity Eaten**: Text input (required) - e.g. "Full bowl", "Half portion"
- **Leftovers Thrown Away**: Text input (optional) - e.g. "None", "Small amount"
- **Food Type**: Text input (required) - e.g. "Wet food - chicken", "Dry kibble"

## Files to Create/Modify
- `shared/graphql/champ-tracker.ts` (extend existing)
- `apps/mobile/champs-tracker/screens/EatingScreen.tsx`
- `apps/mobile/champs-tracker/components/EatingActivityList.tsx`
- `apps/mobile/champs-tracker/screens/HomeScreen.tsx`

## Testing Notes
- Verify form submits correctly and creates backend records
- Test all form fields work as expected
- Confirm activity list displays created eating activities
- Test navigation flow from home → eating → back
- Verify loading states and error handling
- Test with both valid and invalid inputs

## Dependencies
- Backend implementation from ticket 004
- Completed ticket 003 (bathroom frontend pattern to follow)
- Apollo Client setup
- React Navigation

## Estimated Effort
Small - Following established pattern from bathroom activities frontend.