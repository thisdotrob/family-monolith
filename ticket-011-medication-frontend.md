# Ticket 011: Medication Tracking Frontend Implementation

## Overview
Implement the frontend for medication tracking, including both creating medications and logging individual doses. This is more complex than previous activities due to the two-step workflow.

## Acceptance Criteria
- [ ] Medication creation form with all required fields
- [ ] Dose logging form for existing medications
- [ ] GraphQL operations extended for medications and doses
- [ ] Medication list displays active medications
- [ ] Dose history displays logged doses
- [ ] Navigation between medication management and dose logging
- [ ] Loading states and error handling implemented

## Implementation Steps

### 1. Extend GraphQL Operations
Add to existing `shared/graphql/champ-tracker.ts`:
```typescript
export const CREATE_MEDICATION = gql`
  mutation CreateMedication($input: MedicationInput!) {
    createMedication(input: $input) {
      id
      name
      dosage
      startDate
      endDate
      reason
      notes
      createdAt
    }
  }
`;

export const LOG_MEDICATION_DOSE = gql`
  mutation LogMedicationDose($input: MedicationDoseInput!) {
    logMedicationDose(input: $input) {
      id
      medicationId
      doseTimestamp
      notes
      createdAt
    }
  }
`;

export const GET_MEDICATIONS = gql`
  query GetMedications($activeOnly: Boolean) {
    champTracker {
      medications(activeOnly: $activeOnly) {
        id
        userId
        name
        dosage
        startDate
        endDate
        reason
        notes
        createdAt
      }
    }
  }
`;

export const GET_MEDICATION_DOSES = gql`
  query GetMedicationDoses($medicationId: Int, $limit: Int, $offset: Int) {
    champTracker {
      medicationDoses(medicationId: $medicationId, limit: $limit, offset: $offset) {
        id
        medicationId
        userId
        doseTimestamp
        notes
        createdAt
      }
    }
  }
`;
```

### 2. Implement Medication Management Screen
Update `apps/mobile/champs-tracker/screens/MedicationScreen.tsx`:
```typescript
export default function MedicationScreen() {
  // State management for navigation between create/list/dose views
  // Navigation tabs or buttons for different actions
  // Render appropriate component based on current view
}
```

### 3. Create Medication Creation Component
Create `apps/mobile/champs-tracker/components/CreateMedicationForm.tsx`:
```typescript
export default function CreateMedicationForm() {
  // Form state management for medication fields
  // GraphQL mutation hook for creating medication
  // Form with name, dosage, start_date, end_date, reason, notes fields
  // Submit button and validation
}
```

### 4. Create Active Medications List
Create `apps/mobile/champs-tracker/components/MedicationList.tsx`:
```typescript
export default function MedicationList() {
  // GraphQL query hook for active medications
  // List rendering with medication info
  // Button/link to log dose for each medication
  // Loading and error states
}
```

### 5. Create Dose Logging Component
Create `apps/mobile/champs-tracker/components/LogDoseForm.tsx`:
```typescript
export default function LogDoseForm({ medicationId, medicationName }) {
  // Form state for dose logging
  // GraphQL mutation hook for logging dose
  // Simple form with timestamp and notes
  // Submit and success feedback
}
```

### 6. Create Dose History Component
Create `apps/mobile/champs-tracker/components/DoseHistory.tsx`:
```typescript
export default function DoseHistory({ medicationId }) {
  // GraphQL query hook for medication doses
  // List of logged doses with timestamps
  // Loading states and pagination
}
```

### 7. Update Home Screen
Update `apps/mobile/champs-tracker/screens/HomeScreen.tsx`:
- Add medication button with 💊 icon
- Implement navigation to MedicationScreen
- Ensure consistent styling with other activity buttons

## User Workflow
1. **Create Medication**: Name, dosage, dates, reason → Save medication
2. **View Active Medications**: List of current medications
3. **Log Dose**: Select medication → Enter timestamp and notes → Save dose
4. **View Dose History**: See all doses logged for a medication

## Form Fields

### Create Medication Form
- **Name**: Text input (required) - e.g. "Metacam", "Antibiotics"
- **Dosage**: Text input (required) - e.g. "0.5ml", "1 pill"
- **Start Date**: Date picker (required)
- **End Date**: Date picker (optional)
- **Reason**: Text input (required) - e.g. "Pain management", "Infection"
- **Notes**: Multi-line text input (optional)

### Log Dose Form
- **Medication**: Display selected medication name
- **Timestamp**: Date/time picker (default to current time)
- **Notes**: Text input (optional) - e.g. "Given with food", "Difficult to administer"

## Files to Create/Modify
- `shared/graphql/champ-tracker.ts` (extend existing)
- `apps/mobile/champs-tracker/screens/MedicationScreen.tsx`
- `apps/mobile/champs-tracker/components/CreateMedicationForm.tsx`
- `apps/mobile/champs-tracker/components/MedicationList.tsx`
- `apps/mobile/champs-tracker/components/LogDoseForm.tsx`
- `apps/mobile/champs-tracker/components/DoseHistory.tsx`
- `apps/mobile/champs-tracker/screens/HomeScreen.tsx`

## Testing Notes
- Verify medication creation works correctly
- Test dose logging for existing medications
- Confirm medication list shows active medications only
- Test dose history displays correctly
- Verify navigation between different medication views
- Test date inputs work properly
- Ensure error handling for invalid medication IDs

## Dependencies
- Backend implementation from ticket 010
- Completed tickets 003, 005, 007, 009 (established frontend pattern)
- Apollo Client setup
- React Navigation
- Date picker components

## Estimated Effort
Medium - More complex due to multi-step workflow and component relationships.