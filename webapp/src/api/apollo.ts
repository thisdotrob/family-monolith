import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  split,
  from,
} from "@apollo/client";
import { setContext } from "@apollo/client/link/context";
import { getMainDefinition } from "@apollo/client/utilities";
import { onError } from "@apollo/client/link/error";
import { REFRESH_TOKEN_MUTATION } from "../graphql/mutations";
import { isRefreshingTokenVar } from "./state";
import { logout as logoutUser, saveTokens as saveTokensToStorage } from "../auth";

let client: ApolloClient<any>;

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

const errorLink = onError(
  ({ graphQLErrors, operation, forward }) => {
    if (graphQLErrors) {
      for (let err of graphQLErrors) {
        if (err.extensions.code === "TOKEN_EXPIRED") {
          const refreshToken = localStorage.getItem("refreshToken");
          if (!refreshToken) {
            logoutUser(client);
            return;
          }

          isRefreshingTokenVar(true);

          client
            .mutate({
              mutation: REFRESH_TOKEN_MUTATION,
              variables: { refreshToken },
              context: {
                unauthenticated: true,
              },
            })
            .then(({ data }) => {
              const { success, token, refreshToken: newRefreshToken } = data.refreshToken;
              if (success) {
                saveTokensToStorage(client, token, newRefreshToken);
                
                // Retry the failed request
                const oldHeaders = operation.getContext().headers;
                operation.setContext({
                  headers: {
                    ...oldHeaders,
                    authorization: `Bearer ${token}`,
                  },
                });
                
                forward(operation);
              } else {
                logoutUser(client);
              }
            })
            .catch(() => {
              logoutUser(client);
            })
            .finally(() => {
              isRefreshingTokenVar(false);
            });
        }
      }
    }
  }
);

const splitLink = split(
  ({ getContext }) => {
    const { unauthenticated } = getContext();
    return unauthenticated;
  },
  unauthenticatedSchemaHttpLink,
  setAuthHeaderLink.concat(authenticatedSchemaHttpLink)
);

client = new ApolloClient({
  link: from([errorLink, splitLink]),
  cache: new InMemoryCache(),
});

export default client;
