import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  split,
} from "@apollo/client";
import { setContext } from "@apollo/client/link/context";
import { getMainDefinition } from "@apollo/client/utilities";
import AsyncStorage from "@react-native-async-storage/async-storage";

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

const splitLink = split(
  ({ getContext }) => {
    const { unauthenticated } = getContext();
    return unauthenticated;
  },
  authHttpLink,
  authLink.concat(appHttpLink)
);

const client = new ApolloClient({
  link: splitLink,
  cache: new InMemoryCache(),
});

export default client;
