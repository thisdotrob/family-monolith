You are an expert mobile software engineer. Your next task is to configure the Apollo Client for the React Native app to handle communication with the GraphQL backend.

**Commit Title:** `feat(mobileapp): configure basic apollo client`

## 1. Create the Apollo Client Instance

Create a new directory `src/api` and a new file `src/api/apollo.ts`. This will house the Apollo Client configuration. Note the use of the full, absolute URL for the API endpoint as required by the spec for mobile.

**`src/api/apollo.ts`:**
```ts
import { ApolloClient, InMemoryCache, createHttpLink } from '@apollo/client';

const httpLink = createHttpLink({
  uri: 'https://blobfishapp.duckdns.org/graphql',
});

const client = new ApolloClient({
  link: httpLink,
  cache: new InMemoryCache(),
});

export default client;
```

## 2. Provide the Client to the Application

To make the Apollo Client instance available to the entire component tree, you need to wrap the root component with the `ApolloProvider`.

Update `App.tsx` to import the client and the provider.

**`App.tsx`:**
```tsx
import React from 'react';
import { PaperProvider } from 'react-native-paper';
import { SafeAreaView, StyleSheet } from 'react-native';
import { ApolloProvider } from '@apollo/client';
import LoginPage from './src/pages/LoginPage';
import client from './src/api/apollo';

export default function App() {
  return (
    <ApolloProvider client={client}>
      <PaperProvider>
        <SafeAreaView style={styles.container}>
          <LoginPage />
        </SafeAreaView>
      </PaperProvider>
    </ApolloProvider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
});
```

## 3. Verification

Run the application (`npx expo start`). There should be no visible changes. The login form should still be displayed as before. This step confirms that the Apollo Client has been initialized and provided to the application correctly without causing any errors.
