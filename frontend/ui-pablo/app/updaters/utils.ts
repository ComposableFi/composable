import { getAssetByOnChainId } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import BigNumber from "bignumber.js";

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