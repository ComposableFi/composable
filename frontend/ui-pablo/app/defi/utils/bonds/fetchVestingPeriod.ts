import { BondOffer } from "@/defi/types/bonds";
import moment from "moment";

const DEFAULT_BLOCK_TIME = 6 * 1000;

type Props = {
  bondMaturity: BondOffer["maturity"];
  interval: string | undefined;
};

export function fetchVestingPeriod({ interval, bondMaturity }: Props) {
  if (interval) {
    return bondMaturity === "Infinite"
      ? "Infinite"
      : moment(bondMaturity.Finite.returnIn.times(interval).toNumber()).format("d[D] h[H] m[M] s[S]");
  }
  return bondMaturity === "Infinite"
    ? "Infinite"
    : moment(bondMaturity.Finite.returnIn.toNumber() * DEFAULT_BLOCK_TIME).format("d[D] h[H] m[M] s[S]");
}
