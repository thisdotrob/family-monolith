import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  split,
  from,
  gql,
  Observable,
} from "@apollo/client";
import { loadErrorMessages, loadDevMessages } from "@apollo/client/dev";
import { setContext } from "@apollo/client/link/context";
import { onError } from "@apollo/client/link/error";
import { getMainDefinition } from "@apollo/client/utilities";
import AsyncStorage from "@react-native-async-storage/async-storage";
import { REFRESH_TOKEN_MUTATION } from "../../graphql/mutations";

if (__DEV__) {
  // Adds messages only in a dev environment
  loadDevMessages();
  loadErrorMessages();
}

const httpLink = createHttpLink({
  uri: "http://192.168.1.52:4173/v1/graphql",
});

const setAuthHeaderLink = setContext(async (request, prevContext) => {
  const token = await AsyncStorage.getItem("token");
  return {
    headers: {
      ...prevContext.headers,
      authorization: token ? `Bearer ${token}` : "",
    },
  };
});

const createErrorLink = (
  client: ApolloClient<any>,
  setIsAuthenticating: (isAuthenticating: boolean) => void,
  saveTokens: (token: string, refreshToken: string) => Promise<void>,
  logout: () => Promise<void>
) => onError(({ graphQLErrors, networkError, operation, forward }) => {
  if (graphQLErrors && graphQLErrors.some(error => Array.isArray(error) && error.includes("TOKEN_EXPIRED"))) {
    setIsAuthenticating(true);

    return new Observable(observer => {
      (async () => {
        try {
          const refreshToken = await AsyncStorage.getItem("refreshToken");

          if (!refreshToken) {
            throw new Error('No refresh token in AsyncStorage');
          }

          const { data } = await client.mutate({ mutation: REFRESH_TOKEN_MUTATION, variables: { refreshToken }, context: { unauthenticated: true } });

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

let client; // Necessary to present multiple clients being created and then multiple request and refresh attempts made on token expiry.

const createApolloClient = (
  setIsAuthenticating: (isAuthenticating: boolean) => void,
  saveTokens: (token: string, refreshToken: string) => Promise<void>,
  logout: () => Promise<void>
) => {
  if (!client) {
    client = new ApolloClient({ cache: new InMemoryCache() });

    const errorLink = createErrorLink(
      client,
      setIsAuthenticating,
      saveTokens,
      logout
    );

    client.setLink(from([errorLink, setAuthHeaderLink.concat(httpLink)]));
  }

  return client;
};

export { createApolloClient };

