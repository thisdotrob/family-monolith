You are an expert software engineer. Your next task is to configure the Apollo Client to handle communication with the GraphQL backend.

**Commit Title:** `feat(webapp): configure basic apollo client`

## 1. Create the Apollo Client Instance

Create a new directory `src/api` and a new file `src/api/apollo.ts` to house the Apollo Client configuration.

**`src/api/apollo.ts`:**
```ts
import { ApolloClient, InMemoryCache, createHttpLink } from '@apollo/client';

const httpLink = createHttpLink({
  uri: '/graphql',
});

const client = new ApolloClient({
  link: httpLink,
  cache: new InMemoryCache(),
});

export default client;
```

## 2. Provide the Client to the Application

To make the Apollo Client instance available to the entire component tree, you need to wrap the root component with the `ApolloProvider`.

Update `src/main.tsx` to import the client and the provider.

**`src/main.tsx`:**
```tsx
import React from 'react'
import ReactDOM from 'react-dom/client'
import { ApolloProvider } from '@apollo/client'
import App from './App.tsx'
import './index.css'
import client from './api/apollo.ts'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <ApolloProvider client={client}>
      <App />
    </ApolloProvider>
  </React.StrictMode>,
)
```

## 3. Verification

Run the development server (`npm run dev`). There should be no visible changes to the application. The login form should still be displayed as before.

Check the browser's developer console to ensure there are no new errors. This confirms that the Apollo Client has been initialized and provided to the application correctly.
