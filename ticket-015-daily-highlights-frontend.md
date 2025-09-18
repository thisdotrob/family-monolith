# Ticket 015: Daily Highlights Frontend Implementation

## Overview
Implement the frontend for daily highlights tracking, including tag selection and quick notable moment capture functionality.

## Acceptance Criteria
- [ ] Daily highlight entry form with content and tag selection
- [ ] GraphQL operations extended for daily highlights
- [ ] Form submission successfully creates daily highlights
- [ ] Activity list displays recent highlights with tags
- [ ] Tag selection UI with predefined options (funny, milestone, etc.)
- [ ] Loading states and error handling implemented
- [ ] Navigation from home screen to highlights screen working

## Implementation Steps

### 1. Extend GraphQL Operations
Add to existing `shared/graphql/champ-tracker.ts`:
```typescript
export const CREATE_DAILY_HIGHLIGHT = gql`
  mutation CreateDailyHighlight($input: DailyHighlightInput!) {
    createDailyHighlight(input: $input) {
      id
      timestamp
      content
      tags
      createdAt
    }
  }
`;

export const GET_DAILY_HIGHLIGHTS = gql`
  query GetDailyHighlights($limit: Int, $offset: Int) {
    champTracker {
      dailyHighlights(limit: $limit, offset: $offset) {
        id
        userId
        timestamp
        content
        tags
        createdAt
      }
    }
  }
`;
```

### 2. Create Tag Selection Component
Create `apps/mobile/champs-tracker/components/TagSelector.tsx`:
```typescript
export default function TagSelector({ selectedTags, onTagsChange }) {
  // Predefined tag options: funny, milestone, concerning, cute, playful, sleepy, etc.
  // Multi-select UI (checkboxes or chips)
  // Handle tag selection/deselection
  // Visual representation of selected tags
}
```

### 3. Implement Form Component
Update `apps/mobile/champs-tracker/screens/HighlightsScreen.tsx`:
```typescript
export default function HighlightsScreen() {
  // Form state management for highlight fields
  // GraphQL mutation hook
  // Form submission handler
  // UI with timestamp, content (optional), and tag selection
  // Submit button and loading states
  // Quick capture design for fast entry
}
```

### 4. Create Highlights List Component
Create `apps/mobile/champs-tracker/components/DailyHighlightsList.tsx`:
```typescript
export default function DailyHighlightsList() {
  // GraphQL query hook for daily highlights
  // List rendering with highlight content and tags
  // Tag display with visual styling (colored chips/badges)
  // Loading and error states
  // Pull-to-refresh functionality
}
```

### 5. Update Home Screen
Update `apps/mobile/champs-tracker/screens/HomeScreen.tsx`:
- Add daily highlights button with ⭐ icon
- Implement navigation to HighlightsScreen
- Ensure consistent styling with other activity buttons

### 6. Wire Everything Together
- Import and use new daily highlights GraphQL operations
- Connect form submission to backend
- Display success/error messages
- Ensure proper navigation flow

## Form Fields
- **Timestamp**: Date/time picker (default to current time)
- **Content**: Multi-line text input (optional) - e.g. "Champagne learned to open the treat cabinet!"
- **Tags**: Multi-select from predefined options:
  - `funny` - Amusing behaviors or moments
  - `milestone` - Important achievements or firsts
  - `concerning` - Behaviors that might need attention
  - `cute` - Adorable moments worth remembering
  - `playful` - High energy or playful behavior
  - `sleepy` - Peaceful or sleeping moments
  - `social` - Interactions with people or other animals
  - `food` - Food-related behaviors or preferences

## UI Design Notes
- **Quick Entry Focus**: Streamlined form for fast moment capture
- **Tag Visual Design**: Use colored chips/badges for easy recognition
- **Content Optional**: Allow tag-only entries for very quick logging
- **Recent Highlights**: Show recent entries prominently for context

## Files to Create/Modify
- `shared/graphql/champ-tracker.ts` (extend existing)
- `apps/mobile/champs-tracker/screens/HighlightsScreen.tsx`
- `apps/mobile/champs-tracker/components/TagSelector.tsx`
- `apps/mobile/champs-tracker/components/DailyHighlightsList.tsx`
- `apps/mobile/champs-tracker/screens/HomeScreen.tsx`

## Testing Notes
- Verify form submits correctly and creates backend records
- Test tag selection works with multiple tags
- Test submission with content only, tags only, and both
- Confirm highlights list displays with proper tag styling
- Test navigation flow from home → highlights → back
- Verify loading states and error handling
- Test tag serialization matches backend expectations

## Dependencies
- Backend implementation from ticket 014
- Completed tickets 003, 005, 007, 009, 011, 013 (established frontend pattern)
- Apollo Client setup
- React Navigation
- Tag selection UI components

## Estimated Effort
Small - Following established pattern with minor tag selection UI complexity.