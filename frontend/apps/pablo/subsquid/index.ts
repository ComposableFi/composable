import { createClient } from "urql";
import { getEnvironment } from "shared/endpoints";

/**
 * This method is used as single client
 * caches responses
 * @returns urql client
 */
export const makeClient = () =>
  createClient({
    url: getEnvironment("subsquid")
  });
