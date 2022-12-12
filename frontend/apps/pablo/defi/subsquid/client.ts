import { Client, createClient } from "urql";
import { getEnvironment } from "shared/endpoints";

export const singletonClient = createClient({
  url: getEnvironment("subsquid")
});

export function subsquidClient(cached: boolean = false): Client {
  return cached ? singletonClient : createClient({
    url: getEnvironment("subsquid")
  })
}