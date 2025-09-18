# Ticket 009: Vet Visits Frontend Implementation

## Overview
Implement the frontend for vet visit tracking, following the established pattern from previous activity types.

## Acceptance Criteria
- [ ] Vet visit entry form with all required fields
- [ ] GraphQL operations extended for vet visits
- [ ] Form submission successfully creates vet visits
- [ ] Activity list displays recent vet visits
- [ ] Loading states and error handling implemented
- [ ] Navigation from home screen to vet screen working

## Implementation Steps

### 1. Extend GraphQL Operations
Add to existing `shared/graphql/champ-tracker.ts`:
```typescript
export const CREATE_VET_VISIT = gql`
  mutation CreateVetVisit($input: VetVisitInput!) {
    createVetVisit(input: $input) {
      id
      dateTime
      reason
      weightKg
      treatmentsProcedures
      costAmount
      costCurrency
      notes
      createdAt
    }
  }
`;

export const GET_VET_VISITS = gql`
  query GetVetVisits($limit: Int, $offset: Int) {
    champTracker {
      vetVisits(limit: $limit, offset: $offset) {
        id
        userId
        dateTime
        reason
        weightKg
        treatmentsProcedures
        costAmount
        costCurrency
        notes
        createdAt
      }
    }
  }
`;
```

### 2. Implement Form Component
Update `apps/mobile/champs-tracker/screens/VetScreen.tsx`:
```typescript
export default function VetScreen() {
  // Form state management for vet visit fields
  // GraphQL mutation hook
  // Form submission handler
  // UI with date_time, reason, weight_kg, treatments_procedures, cost_amount, cost_currency, notes fields
  // Submit button and loading states
}
```

### 3. Create Activity List Component
Create `apps/mobile/champs-tracker/components/VetVisitList.tsx`:
```typescript
export default function VetVisitList() {
  // GraphQL query hook for vet visits
  // List rendering with vet visit info
  // Loading and error states
  // Pull-to-refresh functionality
}
```

### 4. Update Home Screen
Update `apps/mobile/champs-tracker/screens/HomeScreen.tsx`:
- Add vet visit button with 🏥 icon
- Implement navigation to VetScreen
- Ensure consistent styling with other activity buttons

### 5. Wire Everything Together
- Import and use new vet visit GraphQL operations
- Connect form submission to backend
- Display success/error messages
- Ensure proper navigation flow

## Form Fields
- **Date & Time**: Date/time picker (default to current time)
- **Reason**: Text input (required) - e.g. "Annual checkup", "Vaccination", "Illness"
- **Weight**: Number input for weight in kg (optional)
- **Treatments/Procedures**: Multi-line text input (optional)
- **Cost Amount**: Number input for decimal amounts (optional)
- **Cost Currency**: Text input with default "USD" (optional)
- **Notes**: Multi-line text input (optional)

## Files to Create/Modify
- `shared/graphql/champ-tracker.ts` (extend existing)
- `apps/mobile/champs-tracker/screens/VetScreen.tsx`
- `apps/mobile/champs-tracker/components/VetVisitList.tsx`
- `apps/mobile/champs-tracker/screens/HomeScreen.tsx`

## Testing Notes
- Verify form submits correctly and creates backend records
- Test all form fields work as expected
- Confirm activity list displays created vet visits
- Test navigation flow from home → vet → back
- Verify loading states and error handling
- Test weight and cost fields handle decimal inputs properly
- Test currency field defaults to USD

## Dependencies
- Backend implementation from ticket 008
- Completed tickets 003, 005, 007 (established frontend pattern)
- Apollo Client setup
- React Navigation

## Estimated Effort
Small - Following established pattern from previous activity frontends.