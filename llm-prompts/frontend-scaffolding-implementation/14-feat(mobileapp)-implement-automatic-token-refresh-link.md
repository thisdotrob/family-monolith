You are an expert mobile software engineer. Your final functional task is to implement the automatic token refresh logic for the React Native application.

**Commit Title:** `feat(mobileapp): implement automatic token refresh link`

## 1. Add Refreshing State to AuthContext

Update `src/contexts/AuthContext.tsx` to include an `isRefreshingToken` state and a way to set it.

## 2. Create a Global Loading Overlay

Create a component at `src/components/GlobalLoading.tsx` to block the UI during the refresh.

**`src/components/GlobalLoading.tsx`:**
```tsx
import React from 'react';
import { View, Modal, StyleSheet } from 'react-native';
import { ActivityIndicator, Text } from 'react-native-paper';
import { useAuth } from '../contexts/AuthContext';

const GlobalLoading = () => {
  const { isRefreshingToken } = useAuth();

  return (
    <Modal visible={isRefreshingToken} transparent={true} animationType="fade">
      <View style={styles.container}>
        <View style={styles.content}>
          <ActivityIndicator animating={true} size="large" />
          <Text style={styles.text}>Refreshing session...</Text>
        </View>
      </View>
    </Modal>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: 'rgba(0, 0, 0, 0.5)',
  },
  content: {
    backgroundColor: 'white',
    padding: 20,
    borderRadius: 10,
    alignItems: 'center',
  },
  text: {
    marginTop: 16,
    fontSize: 16,
  },
});

export default GlobalLoading;
```

Render the `GlobalLoading` component in `App.tsx`.

Add a unit test to verify that `App.tsx` still renders successfully.

## 3. Implement the Token Refresh Link

Update `src/api/apollo.ts` to handle token refresh. This will be very similar to the web implementation.

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

The `errors` field isn't currently included in the `Login` mutation so it will need adding.

## 4. Verification

1.  **Simulate Expired Token**: Log in normally. You'll need a way to invalidate the token. One way is to use a proxy to intercept and modify the request, but a simpler way is to have a "test" button on the `HomePage` that manually sets an invalid token in `AsyncStorage` and then triggers a refetch of the `me` query.
2.  **Observe Behavior**:
    *   The `me` query should fail.
    *   The "Refreshing session..." modal should appear.
    *   A `refreshToken` mutation should be sent.
    *   If successful, the modal disappears, and the `HomePage` loads.
    *   If it fails, the user is logged out and returned to the `LoginPage`.
