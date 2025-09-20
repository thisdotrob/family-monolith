# 037 — Mobile: Tasks List (Live) with Grouping, Polling, and Timezone

Spec refs: §§4,6,8,9,11

## Summary
Implement Tasks tab to fetch and display tasks for the selected project with grouping (Overdue/Today/Tomorrow/Upcoming/No date), 10s polling, and timezone variable.

## Scope
- Query tasks with filters (default todo), tz required
- Group into sections by bucket; sort within sections
- Poll every 10s while active; refetch after mutations
- Offline banner disables write controls

## Acceptance Criteria
- Matches grouping and sort rules visually
- Polling stops when app backgrounded and resumes on foreground

## Dependencies
- 010, 050, 051, 052, 033

## Implementation Steps
1) Build list with section headers
2) Wire polling and app state listeners
3) Integrate offline hook
