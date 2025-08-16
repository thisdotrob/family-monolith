import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  split,
  from,
  gql,
  Observable,
} from "@apollo/client";
import { setContext } from "@apollo/client/link/context";
import { onError } from "@apollo/client/link/error";
import { getMainDefinition } from "@apollo/client/utilities";

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

const unauthenticatedSchemaHttpLink = createHttpLink({
  uri: "http://localhost:4173/v1/graphql/auth",
});

const authenticatedSchemaHttpLink = createHttpLink({
  uri: "http://localhost:4173/v1/graphql/app",
});

const setAuthHeaderLink = setContext((_, { headers }) => {
  const token = localStorage.getItem("token");
  return {
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : "",
    },
  };
});

// Global variables to handle refresh state and queuing
let isRefreshing = false;
let failedQueue: Array<{
  resolve: (value?: any) => void;
  reject: (reason?: any) => void;
}> = [];

const processQueue = (error: any, token: string | null = null) => {
  failedQueue.forEach(({ resolve, reject }) => {
    if (error) {
      reject(error);
    } else {
      resolve(token);
    }
  });
  
  failedQueue = [];
};

const createErrorLink = (
  client: ApolloClient<any>,
  setIsRefreshingToken: (isRefreshing: boolean) => void,
  saveTokens: (token: string, refreshToken: string) => void,
  logout: () => void
) => {
  return onError(({ graphQLErrors, networkError, operation, forward }) => {
    if (graphQLErrors) {
      if (graphQLErrors.some(error => error.includes("TOKEN_EXPIRED"))) {
        const refreshToken = localStorage.getItem("refreshToken");
        if (!refreshToken) {
          logout();
          return;
        }

        // Return an Observable to properly handle the async refresh
        return new Observable(observer => {
          if (isRefreshing) {
            // If already refreshing, queue this request
            failedQueue.push({
              resolve: (token: string) => {
                // Update the operation with the new token
                const oldHeaders = operation.getContext().headers;
                operation.setContext({
                  headers: {
                    ...oldHeaders,
                    authorization: `Bearer ${token}`,
                  },
                });
                // Retry the request
                forward(operation).subscribe(observer);
              },
              reject: (error: any) => {
                observer.error(error);
              }
            });
            return;
          }

          isRefreshing = true;
          setIsRefreshingToken(true);

          client
            .mutate({
              mutation: REFRESH_TOKEN_MUTATION,
              variables: { refreshToken },
              context: {
                unauthenticated: true,
              },
            })
            .then(({ data }) => {
              const { success, token, refreshToken: newRefreshToken } =
                data.refreshToken;
              if (success && token && newRefreshToken) {
                saveTokens(token, newRefreshToken);
                
                // Update the current operation with the new token
                const oldHeaders = operation.getContext().headers;
                operation.setContext({
                  headers: {
                    ...oldHeaders,
                    authorization: `Bearer ${token}`,
                  },
                });

                // Process the queue with the new token
                processQueue(null, token);
                
                // Retry the current request
                forward(operation).subscribe(observer);
              } else {
                processQueue(new Error('Token refresh failed'), null);
                logout();
                observer.error(new Error('Token refresh failed'));
              }
            })
            .catch((error) => {
              processQueue(error, null);
              logout();
              observer.error(error);
            })
            .finally(() => {
              isRefreshing = false;
              setIsRefreshingToken(false);
            });
        });
      }
    }

    if (networkError) console.log(`[Network error]: ${networkError}`);
  });
};

const splitLink = split(
  ({ getContext }) => {
    const { unauthenticated } = getContext();
    return unauthenticated;
  },
  unauthenticatedSchemaHttpLink,
  setAuthHeaderLink.concat(authenticatedSchemaHttpLink)
);

const createApolloClient = (
  setIsRefreshingToken: (isRefreshing: boolean) => void,
  saveTokens: (token: string, refreshToken: string) => void,
  logout: () => void
) => {
  const client = new ApolloClient({
    cache: new InMemoryCache(),
  });

  const errorLink = createErrorLink(
    client,
    setIsRefreshingToken,
    saveTokens,
    logout
  );

  client.setLink(from([errorLink, splitLink]));

  return client;
};

export { createApolloClient };
