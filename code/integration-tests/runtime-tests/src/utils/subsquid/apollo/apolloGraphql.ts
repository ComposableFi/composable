import { ApolloClient, InMemoryCache, HttpLink } from "@apollo/client/core";
import fetch from "cross-fetch";

const uri = process.env.SUBSQUID_URL || "http://127.0.0.1:4000/graphql";

export const client = new ApolloClient({
  uri,
  cache: new InMemoryCache(),
  link: new HttpLink({ uri, fetch })
});
