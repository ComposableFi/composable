import BigNumber from "bignumber.js";
import { AssetId } from "@/defi/polkadot/types";

/**
 * Check if pair is valid
 * @param asset1 Asset id | "none"
 * @param asset2 Asset id | "none"
 * @returns boolean
 */
export function isValidAssetPair(asset1: string | "none", asset2: string | "none"): boolean {
    return asset1 !== "none" && asset2 !== "none";
}

export function concatU8a (a: Uint8Array, b: Uint8Array): Uint8Array {
    const c = new Uint8Array(a.length + b.length);
    c.set(a);
    c.set(b, a.length);
    return c;
  }
