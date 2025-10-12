import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  from,
  Observable,
  type NormalizedCacheObject,
} from "@apollo/client";
import { loadErrorMessages, loadDevMessages } from "@apollo/client/dev";
import { setContext } from "@apollo/client/link/context";
import { onError } from "@apollo/client/link/error";
import { REFRESH_TOKEN_MUTATION } from "../graphql/auth";

export type AuthTokens = { token: string | null; refreshToken: string | null };

export type CreateApolloClientDeps = {
  isDev: boolean;
  setIsAuthenticating: (isAuthenticating: boolean) => void;
  getTokens: () => Promise<AuthTokens>;
  saveTokens: (token: string, refreshToken: string) => Promise<void>;
  logout: () => Promise<void>;
  uri?: string;
};

const DEFAULT_GRAPHQL_URI = "https://blobfishapp.duckdns.org/v1/graphql";

const createAuthHeaderLink = (
  getTokens: () => Promise<AuthTokens>,
) =>
  setContext(async (_request: any, prevContext: { headers?: Record<string, string>; unauthenticated?: boolean }) => {
    if (prevContext?.unauthenticated) {
      // Do not attach auth headers for unauthenticated operations (login/refresh)
      return { headers: { ...prevContext.headers } };
    }
    const { token } = await getTokens();
    return {
      headers: {
        ...prevContext.headers,
        authorization: token ? `Bearer ${token}` : '',
      },
    };
  });

const createErrorLink = (
  getClient: () => ApolloClient<any>,
  setIsAuthenticating: (isAuthenticating: boolean) => void,
  getTokens: () => Promise<AuthTokens>,
  saveTokens: (token: string, refreshToken: string) => Promise<void>,
  logout: () => Promise<void>,
) =>
  onError((input: any) => {
    const { graphQLErrors, operation, forward } = input;

    const isUnauthenticatedOp = Boolean(operation?.getContext?.().unauthenticated);

    const hasTokenExpired = Array.isArray(graphQLErrors)
      ? graphQLErrors.some((e: any) => e?.extensions?.code === 'TOKEN_EXPIRED')
      : false;

    if (!isUnauthenticatedOp && hasTokenExpired) {
      setIsAuthenticating(true);

      return new Observable((observer: any) => {
        (async () => {
          try {
            const { refreshToken } = await getTokens();
            if (!refreshToken) throw new Error('No refresh token found');

            const { data } = await getClient().mutate({
              mutation: REFRESH_TOKEN_MUTATION,
              variables: { refreshToken },
              context: { unauthenticated: true },
            });

            const { success, token, refreshToken: newRefreshToken } = data.refreshToken;
            if (!success || !token || !newRefreshToken) {
              throw new Error('Refresh token mutation failed');
            }

            await saveTokens(token, newRefreshToken);

            const oldHeaders = operation.getContext().headers;
            operation.setContext({ headers: { ...oldHeaders, authorization: `Bearer ${token}` } });

            forward(operation).subscribe({
              next: (val: any) => observer.next(val),
              error: (err: any) => observer.error(err),
              complete: () => observer.complete(),
            });
          } catch (err: any) {
            await logout();
            operation.setContext({ headers: { ...(operation.getContext().headers || {}), authorization: '' } });
            observer.error(err);
          } finally {
            setIsAuthenticating(false);
          }
        })();
      });
    }
  });

let client: ApolloClient<NormalizedCacheObject> | null = null; // cache a single client instance

export const createApolloClient = ({
  // Explicit types to keep TS happy when compiled from different projects

  isDev,
  setIsAuthenticating,
  getTokens,
  saveTokens,
  logout,
  uri = DEFAULT_GRAPHQL_URI,
}: CreateApolloClientDeps) => {
  if (client) return client;

  if (isDev) {
    // Adds messages only in a dev environment
    loadDevMessages();
    loadErrorMessages();
  }

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

  const httpLink = createHttpLink({ uri });

  localClient = new ApolloClient({
    cache: new InMemoryCache(),
    link: from([errorLink, authHeaderLink.concat(httpLink)]),
  });

  client = localClient;
  return client;
};
