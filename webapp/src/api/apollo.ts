import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  split,
  from,
  gql,
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

const createErrorLink = (
  client: ApolloClient<any>,
  setIsRefreshingToken: (isRefreshing: boolean) => void,
  saveTokens: (token: string, refreshToken: string) => void,
  logout: () => void
) => {
  return onError(({ graphQLErrors, networkError, operation, forward }) => {
    if (graphQLErrors) {
      for (let err of graphQLErrors) {
        if (err.message.includes("TOKEN_EXPIRED")) {
          const refreshToken = localStorage.getItem("refreshToken");
          if (!refreshToken) {
            logout();
            return;
          }

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
                const oldHeaders = operation.getContext().headers;
                operation.setContext({
                  headers: {
                    ...oldHeaders,
                    authorization: `Bearer ${token}`,
                  },
                });
                // Retry the request
                return forward(operation);
              } else {
                logout();
              }
            })
            .catch(() => {
              logout();
            })
            .finally(() => {
              setIsRefreshingToken(false);
            });
        }
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
