# 036 — Mobile: Tag Manager UI

Spec refs: §§6,8,10,15

## Summary
Provide a Tag Manager accessible from Saved Views or Tasks header to create/rename/delete tags with normalization and merge behavior.

## Scope
- Query tags; Mutations createTag, renameTag, deleteTag
- Normalize inputs on client (trim/collapse/strip #) for UX but rely on server rules

## Acceptance Criteria
- Rename collisions result in merge (as observed after refetch)
- Delete prevented if in use with clear error message

## Dependencies
- 006

## Implementation Steps
1) Build simple list UI with edit/delete
2) Hook up mutations and handle errors
