You are an expert mobile software engineer. Your task is to add the authentication token to the headers of all GraphQL requests and create a simple authenticated view, mirroring the web app's functionality.

**Commit Title:** `feat(mobileapp): implement authenticated apollo link and basic routing`

## 1. Create a Home Page for Authenticated Users

Create a new page at `src/pages/HomePage.tsx`.

**`src/graphql/queries.ts`:**
Create a new file for queries.
```ts
import { gql } from '@apollo/client';

export const ME_QUERY = gql`
  query Me {
    me {
      username
      firstName
    }
  }
`;
```

**`src/pages/HomePage.tsx`:**
```tsx
import React from 'react';
import { View, StyleSheet } from 'react-native';
import { useQuery } from '@apollo/client';
import { useAuth } from '../contexts/AuthContext';
import { ME_QUERY } from '../graphql/queries';
import { Text, Button, ActivityIndicator } from 'react-native-paper';

const HomePage = () => {
  const { logout } = useAuth();
  const { data, loading, error } = useQuery(ME_QUERY);

  if (loading) {
    return <ActivityIndicator animating={true} style={styles.centered} />;
  }
  
  if (error) {
    return <Text style={styles.centered}>Error: {error.message}</Text>;
  }

  return (
    <View style={styles.container}>
      <Text variant="headlineLarge">Welcome, {data?.me?.firstName || 'User'}!</Text>
      <Text variant="bodyLarge" style={styles.username}>Your username is: {data?.me?.username}</Text>
      <Button
        mode="contained"
        onPress={logout}
        style={styles.button}
        icon="logout"
      >
        Logout
      </Button>
    </View>
  );
};

const styles = StyleSheet.create({
  centered: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
  },
  username: {
    marginVertical: 16,
  },
  button: {
    marginTop: 20,
  },
});

export default HomePage;
```

## 2. Implement the Authenticated Apollo Link

Update `src/api/apollo.ts` to include an `authLink` that retrieves the token from `AsyncStorage`. We'll need a `splitLink` now because authenticated GraphQL requests must be sent to `/v1/graphql/app` not `/v1/graphql/auth`. Make sure host is set to `192.168.1.53` so developing with expo works.

**`src/api/apollo.ts`:**
```ts
import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  split,
} from "@apollo/client";
import { setContext } from "@apollo/client/link/context";
import { getMainDefinition } from "@apollo/client/utilities";

const authHttpLink = createHttpLink({
  uri: "http://192.168.1.53:4173/v1/graphql/auth",
});

const appHttpLink = createHttpLink({
  uri: "http://192.168.1.53:4173/v1/graphql/app",
});

const authLink = setContext((_, { headers }) => {
  const token = await AsyncStorage.getItem('token');
  return {
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : "",
    },
  };
});

const splitLink = split(
  ({ getContext }) => {
    const { unauthenticated } = getContext();
    return unauthenticated;
  },
  authHttpLink,
  authLink.concat(appHttpLink)
);

const client = new ApolloClient({
  link: splitLink,
  cache: new InMemoryCache(),
});

export default client;
```


## 3. Implement Basic Routing

Create a new root component `src/components/Router.tsx` to handle the navigation logic.

**`src/components/Router.tsx`:**
```tsx
import React from 'react';
import { useAuth } from '../contexts/AuthContext';
import HomePage from '../pages/HomePage';
import LoginPage from '../pages/LoginPage';
import { ActivityIndicator, View, StyleSheet } from 'react-native';

const Router = () => {
  const { token, isLoading } = useAuth();

  if (isLoading) {
    return (
      <View style={styles.centered}>
        <ActivityIndicator size="large" />
      </View>
    );
  }

  return token ? <HomePage /> : <LoginPage />;
};

const styles = StyleSheet.create({
  centered: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
});

export default Router;
```

**`App.tsx`:**
Update the main App component to use the new `Router`.
```tsx
import React from 'react';
import { PaperProvider } from 'react-native-paper';
import { SafeAreaView, StyleSheet } from 'react-native';
import { ApolloProvider } from '@apollo/client';
import { AuthProvider } from './src/contexts/AuthContext';
import client from './src/api/apollo';
import Router from './src/components/Router';

export default function App() {
  return (
    <AuthProvider>
      <ApolloProvider client={client}>
        <PaperProvider>
          <SafeAreaView style={styles.container}>
            <Router />
          </SafeAreaView>
        </PaperProvider>
      </ApolloProvider>
    </AuthProvider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
});
```

## 4. Verification

1.  Run the application. It should show the `LoginPage`.
2.  Log in with correct credentials.
3.  Upon success, you should be navigated to the `HomePage`, which should display your name.
4.  Click "Logout". You should be returned to the `LoginPage`.
5.  Fully close and restart the app. It should briefly show a loading indicator and then go directly to the `HomePage`, demonstrating that the session is persisted.
