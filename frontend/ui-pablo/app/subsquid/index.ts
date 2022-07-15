import { createClient } from 'urql';

export const subsquidClient = createClient({
  url: process.env.SUBSQUID_URL || "",
});
/**
 * This method is used as single client
 * caches responses
 * @returns urql client
 */
export const makeClient = () => createClient({
  url: process.env.SUBSQUID_URL || "",
})