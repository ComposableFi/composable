import { concatU8a } from "./u8a";
import { ApiPromise } from "@polkadot/api";
import { PALLET_TYPE_ID } from "./constants";

export function getSubAccount(api: ApiPromise, poolId: string) {
  const palletId = api.consts.pablo.palletId.toU8a();
  const poolAccountId = api.createType("([u8; 4], [u8; 8], u64)", [
    PALLET_TYPE_ID,
    palletId,
    poolId,
  ]);

  const accountIdu8a = poolAccountId.toU8a();
  const poolAccount = concatU8a(
    accountIdu8a,
    new Uint8Array(32 - accountIdu8a.length).fill(0)
  );
  return api.createType("AccountId32", poolAccount).toString();
}
