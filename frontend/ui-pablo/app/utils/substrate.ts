import { ApiPromise } from "@polkadot/api";
import { u8aToHex, stringToU8a } from "@polkadot/util";

export function concatU8a (a: Uint8Array, b: Uint8Array): Uint8Array {
  const c = new Uint8Array(a.length + b.length);
  c.set(a);
  c.set(b, a.length);
  return c;
}

const PALLET_TYPE_ID = "modl";
/* See task https://app.clickup.com/t/2u9un3m
 * how to create AccountId for derived Accounts
 * within a pallet
 */
export function createPoolAccountId(parachainApi: ApiPromise, poolId: number): any {
  const palletTypeId = stringToU8a(PALLET_TYPE_ID);
  const palletId = parachainApi.consts.pablo.palletId.toU8a();
  const poolAccountId = parachainApi.createType("([u8; 4], [u8; 8], u64)", [
    palletTypeId,
    palletId,
    poolId
  ])

  const accountIdu8a = poolAccountId.toU8a();
  const poolAccount = concatU8a(
    accountIdu8a,
    new Uint8Array(32 - accountIdu8a.length).fill(0)
  );

  return u8aToHex(poolAccount)
}