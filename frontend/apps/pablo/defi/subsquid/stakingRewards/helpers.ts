import { makeClient } from "../makeClient";

export async function fetchSubsquid<T>(query: string): Promise<T> {
  try {
    const queryResponse = await makeClient().query(query).toPromise();
    const { error, data } = queryResponse;

    if (error) throw new Error(error.message);
    if (!data) throw new Error(`[fetchSubsquid] unable to fetch data.`);

    return data;
  } catch (error: any) {
    console.error(error.message);
    return Promise.reject(error);
  }
}
