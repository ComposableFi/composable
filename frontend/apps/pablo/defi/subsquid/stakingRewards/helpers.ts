import { subsquidClient } from "../client";

export async function fetchSubsquid<T>(query: string, cached: boolean = false): Promise<T> {
  try {
    const queryResponse = await subsquidClient(cached).query(query).toPromise();
    const { error, data } = queryResponse;

    if (error) throw new Error(error.message);
    if (!data) throw new Error(`[fetchSubsquid] unable to fetch data.`);

    return data;
  } catch (error: any) {
    console.error(error.message);
    return Promise.reject(error);
  }
}
