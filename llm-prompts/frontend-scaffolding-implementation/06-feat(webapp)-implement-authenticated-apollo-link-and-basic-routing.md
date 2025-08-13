You are an expert software engineer. Your task is to add the authentication token to the headers of all GraphQL requests and create a simple authenticated view.

**Commit Title:** `feat(webapp): implement authenticated apollo link and basic routing`

## 1. Create a Home Page for Authenticated Users

Create a new page at `src/pages/HomePage.tsx` that will be shown only to logged-in users. This page will perform a simple query to fetch the current user's data.

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
import { useQuery } from '@apollo/client';
import { useAuth } from '../contexts/AuthContext';
import { ME_QUERY } from '../graphql/queries';

const HomePage = () => {
  const { logout } = useAuth();
  const { data, loading, error } = useQuery(ME_QUERY);

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error.message}</p>;

  return (
    <div className="min-h-screen bg-gray-100 flex items-center justify-center">
      <div className="bg-white p-8 rounded-lg shadow-md w-full max-w-md text-center">
        <h1 className="text-2xl font-bold mb-6">Welcome, {data?.me?.firstName || 'User'}!</h1>
        <p>Your username is: {data?.me?.username}</p>
        <button
          onClick={logout}
          className="mt-6 bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
        >
          Logout
        </button>
      </div>
    </div>
  );
};

export default HomePage;
```

## 2. Implement the Authenticated Apollo Link

Update `src/api/apollo.ts` to include an `authLink` that retrieves the token from `localStorage` and adds it to the `Authorization` header of every request.

**`src/api/apollo.ts`:**
```ts
import { ApolloClient, InMemoryCache, createHttpLink } from '@apollo/client';
import { setContext } from '@apollo/client/link/context';

const httpLink = createHttpLink({
  uri: '/graphql',
});

const authLink = setContext((_, { headers }) => {
  // get the authentication token from local storage if it exists
  const token = localStorage.getItem('token');
  // return the headers to the context so httpLink can read them
  return {
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : "",
    }
  }
});

const client = new ApolloClient({
  link: authLink.concat(httpLink),
  cache: new InMemoryCache(),
});

export default client;
```

## 3. Implement Basic Routing

Update `src/App.tsx` to show the `HomePage` if the user is authenticated (i.e., has a token) and the `LoginPage` otherwise.

**`src/App.tsx`:**
```tsx
import { useAuth } from './contexts/AuthContext';
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';

function App() {
  const { token } = useAuth();

  return token ? <HomePage /> : <LoginPage />;
}

export default App;
```

## 4. Verification

1.  Run the application (`npm run dev`). You should see the `LoginPage`.
2.  Log in with correct credentials.
3.  Upon successful login, you should be redirected to the `HomePage`. The page should display a welcome message with your first name.
4.  Open the browser's network inspector. You should see a GraphQL request for the `me` query, and its request headers should include the `Authorization: Bearer <token>` header.
5.  Click the "Logout" button. You should be returned to the `LoginPage`.
6.  Refresh the page while logged in. You should remain on the `HomePage`, demonstrating that the token is being correctly read from `localStorage` on initial load.
