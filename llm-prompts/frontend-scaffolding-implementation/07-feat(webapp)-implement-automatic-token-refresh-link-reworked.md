You are an expert software engineer. This is a complex but crucial step. Your task is to implement the automatic token refresh logic in the `webapp` workspace. When a token has expired, this monolith's server responds with 401 and the following in the response body:

```
{
  "success": false,
  "errors": [
    [
      "TOKEN_EXPIRED",
      "token has expired"
    ]
  ]
}
```

When this occurs, the webapp should refresh the jwt using a refresh token graphql mutation sent to the server's unauthenticated graphql schema at http://localhost:4173/v1/graphql/auth

The refresh token mutation is implemented on the server in @server/src/graphql/auth.rs


**Commit Title:** `feat(webapp): implement automatic token refresh`

## 1. Add a Refreshing State to AuthContext

First, we need a way to signal to the UI that a token refresh is in progress.

**`@webapp/src/contexts/AuthContext.tsx`:**
Update the `AuthProvider` to include an `isRefreshingToken` state and `setIsRefreshingToken` setter for that state.

## 2. Create a Loading Overlay Component

Create a component to block the UI during the refresh. It should use the `isRefreshingToken` state from step 1 to know when to block the UI. It should block the UI and display 'Refreshing JWT, hold tight...'. Use this component in `@webapp/src/App.tsx`.

## 3. Implement the `refreshToken` mutation

**`@webapp/src/graphql/mutations.ts`:**
Add the `refreshToken` mutation to the webapp. Check for compatibility with the backend schema in `@server/src/graphql`.

## 4. Update webapp `client` to refresh token and resend original request

Update the webapp's Apollo client to perform the `refreshToken` mutation when a `TOKEN_EXPIRED` response is received for a query. After refreshing the token, the original request should be attempted again.

## 5. Set `isRefreshingToken`

Set it to `true` whilst the `refreshToken` mutation is being made, and back to `false` once it has succeeded.

## 6. Write some unit tests that cover steps 1-5.

## 7. Manual verification

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

