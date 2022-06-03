import { createClient } from 'urql';

export const subsquidClient = createClient({
  url: process.env.SUBSQUID_URL || "",
});