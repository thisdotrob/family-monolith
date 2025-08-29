import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  from,
  Observable,
} from "@apollo/client";
import { loadErrorMessages, loadDevMessages } from "@apollo/client/dev";
import { setContext } from "@apollo/client/link/context";
import { onError } from "@apollo/client/link/error";
import { getMainDefinition } from "@apollo/client/utilities";
import { REFRESH_TOKEN_MUTATION } from "../graphql/mutations";

if (__DEV__) {
  // Adds messages only in a dev environment
  loadDevMessages();
  loadErrorMessages();
}

const httpLink = createHttpLink({
  uri: "http://192.168.1.52:4173/v1/graphql",
});

type AuthTokens = { token: string | null; refreshToken: string | null };

const createAuthHeaderLink = (
  getTokens: () => Promise<AuthTokens>,
) => setContext(async (request, prevContext) => {
  const { token } = await getTokens();
  return {
    headers: {
      ...prevContext.headers,
      authorization: token ? `Bearer ${token}` : "",
    },
  };
});

const createErrorLink = (
  getClient: () => ApolloClient<any>,
  setIsAuthenticating: (isAuthenticating: boolean) => void,
  getTokens: () => Promise<AuthTokens>,
  saveTokens: (token: string, refreshToken: string) => Promise<void>,
  logout: () => Promise<void>
) => onError(({ graphQLErrors, networkError, operation, forward }) => {
  if (graphQLErrors && graphQLErrors.some(error => Array.isArray(error) && error.includes("TOKEN_EXPIRED"))) {
    setIsAuthenticating(true);

    return new Observable(observer => {
      (async () => {
        try {
          const { refreshToken } = await getTokens();

          if (!refreshToken) {
            throw new Error('No refresh token found');
          }

          const { data } = await getClient().mutate({ mutation: REFRESH_TOKEN_MUTATION, variables: { refreshToken }, context: { unauthenticated: true } });

          const { success, token, refreshToken: newRefreshToken } = data.refreshToken;

          if (!success) {
            throw new Error('Refresh token mutation failed');
          }

          await saveTokens(token, newRefreshToken);

          const oldHeaders = operation.getContext().headers;

          operation.setContext({ headers: { ...oldHeaders, authorization: `Bearer ${token}` } });

          forward(operation).subscribe(observer);
        } catch (err) {
          await logout();

          observer.error(err);
        }
      })();
    });
  }
});

import type { NormalizedCacheObject } from '@apollo/client';

let client: ApolloClient<NormalizedCacheObject> | null = null; // cache a single client instance

const createApolloClient = (
  setIsAuthenticating: (isAuthenticating: boolean) => void,
  getTokens: () => Promise<AuthTokens>,
  saveTokens: (token: string, refreshToken: string) => Promise<void>,
  logout: () => Promise<void>
) => {
  if (!client) {
    let localClient: ApolloClient<NormalizedCacheObject>;

    const errorLink = createErrorLink(
      () => localClient,
      setIsAuthenticating,
      getTokens,
      saveTokens,
      logout
    );

    const authHeaderLink = createAuthHeaderLink(
      getTokens,
    );

    localClient = new ApolloClient({
      cache: new InMemoryCache(),
      link: from([errorLink, authHeaderLink.concat(httpLink)]),
    });

    client = localClient;
  }

  return client;
};

export { createApolloClient };
