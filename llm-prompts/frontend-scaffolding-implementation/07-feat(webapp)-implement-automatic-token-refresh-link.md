You are an expert software engineer. This is a complex but crucial step. Your task is to implement the automatic token refresh logic using an Apollo Link.

**Commit Title:** `feat(webapp): implement automatic token refresh link`

## 1. Add a Refreshing State to AuthContext

Update `src/contexts/AuthContext.tsx` to include an `isRefreshingToken` state and a way to set it.

## 2. Create a Global Loading Overlay

Create a component at `src/components/GlobalLoading.tsx` to block the UI during the refresh:

```tsx
import React from 'react';
import { useAuth } from '../contexts/AuthContext';

const GlobalLoading = () => {
  const { isRefreshingToken } = useAuth();

  if (!isRefreshingToken) {
    return null;
  }

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center">
      <div className="bg-white p-6 rounded-lg shadow-xl">
        <p className="text-lg font-semibold">Refreshing session...</p>
      </div>
    </div>
  );
};

export default GlobalLoading;
```

Render the `GlobalLoading` component in `App.tsx`.

Make sure the unit tests verify that `App.tsx` still renders successfully.

## 3. Implement the Token Refresh Link

Update `src/api/apollo.ts` to handle token refresh.

Add the `refreshToken` mutation:
```ts
export const REFRESH_TOKEN_MUTATION = gql`
  mutation RefreshToken($refreshToken: String!) {
    refreshToken(input: { refreshToken: $refreshToken }) {
      success
      token
      refreshToken
      errors
    }
  }
`;
```

Update the Apollo client to refresh the token when an expired token response is received to a GraphQL query. An expired token response will include `TOKEN_EXPIRED` in `errors`.

## 4. Verification

This is the most difficult part to test.

1.  **Simulate Expired Token**:
    *   Log in to the application normally.
    *   Go into your browser's developer tools.
    *   In `localStorage`, manually change the `token` to an invalid value (e.g., `eyJhb...invalid`).
    *   Refresh the `HomePage`.
2.  **Observe Behavior**:
    *   The `me` query on the `HomePage` should fail with an "Unauthorized" error.
    *   The UI should be blocked by the "Refreshing session..." overlay.
    *   The application should send a `refreshToken` mutation in the background.
    *   If the refresh is successful, the overlay should disappear, the `me` query should be retried (this time with a valid token), and the page should load correctly.
    *   If the `refreshToken` is also invalid or expired, you should be logged out and redirected to the `LoginPage`.
