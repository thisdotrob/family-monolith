import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  split,
  from,
  Observable,
} from "@apollo/client";
import { setContext } from "@apollo/client/link/context";
import { onError } from "@apollo/client/link/error";
import { getMainDefinition } from "@apollo/client/utilities";
import AsyncStorage from "@react-native-async-storage/async-storage";
import { REFRESH_TOKEN_MUTATION } from "../../graphql/mutations";
import { refreshTokenStateManager } from "../../src/api/refreshTokenState";

const authHttpLink = createHttpLink({
  uri: "http://192.168.1.53:4173/v1/graphql/auth",
});

const appHttpLink = createHttpLink({
  uri: "http://192.168.1.53:4173/v1/graphql/app",
});

const authLink = setContext(async (_, { headers }) => {
  const token = await AsyncStorage.getItem("token");
  return {
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : "",
    },
  };
});

let pendingRequests: (() => void)[] = [];

const resolvePendingRequests = () => {
  pendingRequests.forEach((callback) => callback());
  pendingRequests = [];
};

const errorLink = onError(({ graphQLErrors, operation, forward }) => {
  if (graphQLErrors) {
    if (graphQLErrors.some(errors => errors.includes("TOKEN_EXPIRED"))) {
      if (operation.operationName === "RefreshToken") {
        // Don't get stuck in a loop trying to refresh the refresh token.
        return;
      }

      if (refreshTokenStateManager.get()) {
        return new Observable((observer) => {
          pendingRequests.push(() => {
            forward(operation).subscribe(observer);
          });
        });
      }

      refreshTokenStateManager.set(true);

      return new Observable((observer) => {
        AsyncStorage.getItem("refreshToken")
          .then((refreshToken) => {
            if (!refreshToken) {
              throw new Error("No refresh token available.");
            }

            const refreshClient = new ApolloClient({
              link: authHttpLink,
              cache: new InMemoryCache(),
            });

            return refreshClient.mutate({
              mutation: REFRESH_TOKEN_MUTATION,
              variables: { refreshToken },
            });
          })
          .then(async ({ data }) => {
            const {
              success,
              token,
              refreshToken: newRefreshToken,
              errors,
            } = data.refreshToken;

            if (!success || !token || !newRefreshToken) {
              console.error("Failed to refresh token:", errors);
              throw new Error("Failed to refresh token.");
            }

            await AsyncStorage.setItem("token", token);
            await AsyncStorage.setItem("refreshToken", newRefreshToken);

            resolvePendingRequests();
            forward(operation).subscribe(observer);
          })
          .catch(async (error) => {
            console.error("Token refresh failed:", error);
            await AsyncStorage.removeItem("token");
            await AsyncStorage.removeItem("refreshToken");
            // The UI should react to the cleared tokens via AuthContext.
            // We just forward the error to the operation that failed.
            observer.error(error);
          })
          .finally(() => {
            refreshTokenStateManager.set(false);
          });
      });
    }
  }

  return forward(operation);
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
  link: from([errorLink, splitLink]),
  cache: new InMemoryCache(),
});

export default client;
