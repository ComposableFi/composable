import { createClient } from 'urql';
/**
 * This method is used as single client
 * caches responses
 * @returns urql client
 */
export const makeClient = () => createClient({
  url: process.env.SUBSQUID_URL || "",
})