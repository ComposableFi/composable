import { AssetId } from "@/defi/polkadot/types";
import BigNumber from "bignumber.js";

/* See task https://app.clickup.com/t/2u9un3m
 * how to create AccountId for derived Accounts
 * within a pallet
 */
export function createPoolAccountId(poolId: number): string {
    enum PalletIds {
      PalletsId = "0x6d6f646c",
      Pablo = "70616c6c5f706162",
    }

    const end = new Array(27).fill("0").join("");
    const start = new Array(13).fill("0").join("");
    // JS eats the leading zero below 10
    let poolIdStr = poolId < 16 ? "0" + poolId.toString(16) : poolId.toString(16)
    const startWithId = (poolIdStr + start).substring(0, 13);

    return (
      PalletIds.PalletsId.toString() +
      PalletIds.Pablo.toString() +
      (startWithId + end)
    ).substring(0, 66);
}
/**
 * Check if pair is valid
 * @param asset1 Asset id | "none"
 * @param asset2 Asset id | "none"
 * @returns boolean
 */
export function isValidAssetPair(asset1: AssetId | "none", asset2: AssetId | "none"): boolean {
    return asset1 !== "none" && asset2 !== "none";
}

export function toTokenUnits(amount: number | string, decimals: number = 12): BigNumber {
  return new BigNumber(amount).times(new BigNumber(10).pow(decimals))
}