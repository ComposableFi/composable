import { ApolloClient, InMemoryCache } from "@apollo/client/core";

export const client = new ApolloClient({
  uri: process.env.SUBSQUID_URL,
  cache: new InMemoryCache()
});
