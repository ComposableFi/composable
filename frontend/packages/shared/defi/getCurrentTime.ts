import { ApiPromise } from "@polkadot/api";

export async function getCurrentTime(parachainApi: ApiPromise) {
  return Number((await parachainApi.query.timestamp.now()).toString());
}
