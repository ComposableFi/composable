import { ApiPromise } from "@polkadot/api";

export async function getCurrentBlock(parachainApi: ApiPromise) {
  return Number((await parachainApi.query.system.number()).toString());
}
