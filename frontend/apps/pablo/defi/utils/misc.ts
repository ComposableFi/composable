import { ChartRange } from "./charts";
import {
  FORMAT_1D,
  FORMAT_1w,
  FORMAT_1M,
} from "./constants"

/**
 * Check if pair is valid
 * @param asset1 Asset id | "none"
 * @param asset2 Asset id | "none"
 * @returns boolean
 */
export function isValidAssetPair(
  asset1: string | "none",
  asset2: string | "none"
): boolean {
  return asset1 !== "none" && asset2 !== "none";
}

export function concatU8a(a: Uint8Array, b: Uint8Array): Uint8Array {
  const c = new Uint8Array(a.length + b.length);
  c.set(a);
  c.set(b, a.length);
  return c;
}

export function compareU8a(a: Uint8Array, b: Uint8Array): boolean {
  if (a.length !== b.length) return false;

  let equal = true;
  
  a.forEach((a, i) => {
    if (a != b[i]) {
      equal = false
    }
  })

  return equal;
}

export function toMomentChartLabel(chartRange: ChartRange): string {
  return {
    "24h": FORMAT_1D,
    "1w": FORMAT_1w,
    "1m": FORMAT_1M
  }[chartRange];
}