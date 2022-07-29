import { createClient } from "contentful";

const client = () => {
  console.log(process.env.NEXT_PUBLIC_CF_SPACE_ID);
  if (process.env.NEXT_PUBLIC_CF_SPACE_ID && process.env.NEXT_PUBLIC_CF_DELIVERY_ACCESS_TOKEN) {
    return createClient({
      space: process.env.NEXT_PUBLIC_CF_SPACE_ID, // ID of a Compose-compatible space to be used \
      accessToken: process.env.NEXT_PUBLIC_CF_DELIVERY_ACCESS_TOKEN, // delivery API key for the space \
    });
  }

  throw new Error("No Contentful credentials found. Please set the environment variables.");
};

type GetEntryParams = {
  id: string;
  query?: any;
};

export async function getEntry(params: GetEntryParams) {
  try {
    const entry = await client().getEntry(params.id);
    return entry;
  } catch (e: unknown) {
    console.error(e);
  }
  
}
