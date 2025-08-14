import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  split,
} from "@apollo/client";
import { setContext } from "@apollo/client/link/context";
import { getMainDefinition } from "@apollo/client/utilities";

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

const splitLink = split(
  ({ getContext }) => {
    const { unauthenticated } = getContext();
    return unauthenticated;
  },
  unauthenticatedSchemaHttpLink,
  setAuthHeaderLink.concat(authenticatedSchemaHttpLink)
);

const client = new ApolloClient({
  link: splitLink,
  cache: new InMemoryCache(),
});

export default client;
