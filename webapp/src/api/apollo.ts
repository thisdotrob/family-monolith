import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  from,
  ApolloLink,
  fromPromise,
} from '@apollo/client';
import { setContext } from '@apollo/client/link/context';
import { onError } from '@apollo/client/link/error';
import { REFRESH_TOKEN_MUTATION } from '../graphql/mutations';

// A separate client for the refresh token mutation to avoid link loops
const refreshClient = new ApolloClient({
  uri: 'http://localhost:4173/v1/graphql/auth',
  cache: new InMemoryCache(),
});

const httpLink = createHttpLink({
  uri: 'http://localhost:4173/v1/graphql/app',
});

const authLink = setContext((_, { headers }) => {
  const token = localStorage.getItem('token');
  return {
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : '',
    },
  };
});

const errorLink = (
  setIsRefreshing: (isRefreshing: boolean) => void,
  logout: () => Promise<void>
): ApolloLink =>
  onError(({ graphQLErrors, networkError, operation, forward }) => {
    if (graphQLErrors) {
      for (const err of graphQLErrors) {
        // Check for a specific error message or code that indicates an expired token
        if (err.message.includes('Authentication required')) {
          const refreshToken = localStorage.getItem('refreshToken');
          if (!refreshToken) {
            logout();
            return;
          }

          setIsRefreshing(true);

          return fromPromise(
            refreshClient
              .mutate({
                mutation: REFRESH_TOKEN_MUTATION,
                variables: { refreshToken },
              })
              .then(({ data }) => {
                const newTokens = data?.refreshToken;
                if (newTokens?.success && newTokens.token && newTokens.refreshToken) {
                  localStorage.setItem('token', newTokens.token);
                  localStorage.setItem('refreshToken', newTokens.refreshToken);
                  return newTokens.token;
                } else {
                  // Throw an error to be caught by the catch block
                  throw new Error('Failed to refresh token');
                }
              })
          ).flatMap(accessToken => {
            setIsRefreshing(false);
            // Retry the failed request with the new token
            const oldHeaders = operation.getContext().headers;
            operation.setContext({
              headers: {
                ...oldHeaders,
                authorization: `Bearer ${accessToken}`,
              },
            });
            return forward(operation);
          }).catch((e) => {
            console.error(e);
            setIsRefreshing(false);
            logout();
            // We must not re-throw, as that would be an unhandled error.
            // We can return a new observable that errors out, but it's simpler
            // to just complete the observable chain without emitting anything.
            // The logout action will trigger a UI update.
            return fromPromise(new Promise(() => {})); // An observable that never emits
          });
        }
      }
    }

    if (networkError) console.log(`[Network error]: ${networkError}`);
  });

// The main client factory
export const createApolloClient = (
  setIsRefreshing: (isRefreshing: boolean) => void,
  baseLogout: () => void
) => {
  let client: ApolloClient<any>;

  const logout = async () => {
    baseLogout();
    if (client) {
      await client.clearStore();
    }
  };

  client = new ApolloClient({
    link: from([errorLink(setIsRefreshing, logout), authLink, httpLink]),
    cache: new InMemoryCache(),
  });

  return client;
};
