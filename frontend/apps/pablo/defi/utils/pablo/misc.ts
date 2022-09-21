import { ApiPromise } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";
import { PALLET_TYPE_ID } from "../constants";
import { concatU8a } from "../misc";

/* See task https://app.clickup.com/t/2u9un3m
 * how to create AccountId for derived Accounts
 * within a pallet
 */
export function createPabloPoolAccountId(
  parachainApi: ApiPromise,
  poolId: number
): any {
  const palletId = parachainApi.consts.pablo.palletId.toU8a();
  const poolAccountId = parachainApi.createType("([u8; 4], [u8; 8], u64)", [
    PALLET_TYPE_ID,
    palletId,
    poolId,
  ]);

  const accountIdu8a = poolAccountId.toU8a();
  const poolAccount = concatU8a(
    accountIdu8a,
    new Uint8Array(32 - accountIdu8a.length).fill(0)
  );

  return u8aToHex(poolAccount);
}
