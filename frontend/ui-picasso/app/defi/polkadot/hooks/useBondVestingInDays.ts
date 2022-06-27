import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { useBlockInterval } from "@/defi/polkadot/hooks/useBlockInterval";
import BigNumber from "bignumber.js";

export function secondsToDHMS(seconds: number) {
  seconds = Number(seconds);
  const d = Math.floor(seconds / (3600 * 24));
  const h = Math.floor((seconds % (3600 * 24)) / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = Math.floor(seconds % 60);

  function getSuffix(value: number) {
    return value > 1 ? "s" : "";
  }

  const dDisplay = d > 0 ? d + " Day" : "";
  const hDisplay = h > 0 ? h + " Hour" : "";
  const mDisplay = m > 0 ? m + " Minute" : "";
  const sDisplay = s > 0 ? s + " Second" : "";
  return {
    d,
    h,
    m,
    s,
    dDisplay: dDisplay + getSuffix(d),
    hDisplay: hDisplay + getSuffix(h),
    mDisplay: mDisplay + getSuffix(m),
    sDisplay: sDisplay + getSuffix(s),
  };
}

function maturityToSeconds(
  maturity: number | "Infinite",
  interval?: BigNumber
) {
  const DEFAULT_BLOCK_TIME = (interval?.toNumber() ?? 6000_000) / 1000; // 6 seconds as default block time
  return maturity === "Infinite" ? "Infinite" : maturity * DEFAULT_BLOCK_TIME;
}

export function useBondVestingInDays(bondOffer: BondOffer) {
  const interval = useBlockInterval();

  return maturityToSeconds(bondOffer.maturity, interval);
}
