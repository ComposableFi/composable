import { gql as gqlCore, gql as gqlReact } from "@apollo/client/core";

const isFrontend = false;
const gql = isFrontend ? gqlCore : gqlReact;

export default gql;