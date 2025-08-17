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

const httpLink = createHttpLink({
  uri: "http://localhost:4173/v1/graphql",
});

const setAuthHeaderLink = setContext((request, prevContext) => {
  const token = localStorage.getItem("token");
  return {
    headers: {
      ...prevContext.headers,
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
      if (graphQLErrors.some(error => error.includes("TOKEN_EXPIRED"))) {
        const refreshToken = localStorage.getItem("refreshToken");

        if (!refreshToken) {
          logout();
          return;
        } else {
          setIsRefreshingToken(true);
        }

        return new Observable(observer => {
          client
            .mutate({
              mutation: REFRESH_TOKEN_MUTATION,
              variables: { refreshToken },
              context: { unauthenticated: true },
            })
            .then(({ data }) => {
              const { success, token, refreshToken: newRefreshToken } = data.refreshToken;
              if (success && token && newRefreshToken) {
                saveTokens(token, newRefreshToken);
                const oldHeaders = operation.getContext().headers;
                operation.setContext({
                  headers: {
                    ...oldHeaders,
                    authorization: `Bearer ${token}`,
                  },
                });
                forward(operation).subscribe(observer);
              } else {
                logout();
                observer.error(new Error('Token refresh failed'));
              }
            })
            .catch((error) => {
              logout();
              observer.error(error);
            })
            .finally(() => {
              setIsRefreshingToken(false);
            });
        });
      }
    }
  });
};

let client; // Necessary to present multiple clients being created and then multiple request and refresh attempts made on token expiry.

const createApolloClient = (
  setIsRefreshingToken: (isRefreshing: boolean) => void,
  saveTokens: (token: string, refreshToken: string) => void,
  logout: () => void
) => {
  if (!client) {
    client = new ApolloClient({ cache: new InMemoryCache() });

    const errorLink = createErrorLink(
      client,
      setIsRefreshingToken,
      saveTokens,
      logout
    );

    client.setLink(from([errorLink, setAuthHeaderLink.concat(httpLink)]));
  }

  return client;
};

export { createApolloClient };
