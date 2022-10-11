import { ApolloClient, InMemoryCache } from "@apollo/client";
import { getEnvironment } from "shared/endpoints";

export const client = new ApolloClient({
  uri: getEnvironment("subsquid"),
  cache: new InMemoryCache()
});
