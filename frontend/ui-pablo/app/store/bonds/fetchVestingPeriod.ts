import moment from "moment";
import { BondOffer } from "./bonds.types";

const DEFAULT_BLOCK_TIME = 6 * 1000;

type Props = {
  bondMaturity: BondOffer["maturity"];
  interval: number | undefined;
};

export function fetchVesitngPeriod({ interval, bondMaturity }: Props) {
  if (interval) {
    return bondMaturity === "Infinite"
      ? "Infinite"
      : moment(bondMaturity * interval).format("d[D] h[H] m[M] s[S]");
  }
  return bondMaturity
    ? "Infinite"
    : moment(bondMaturity * DEFAULT_BLOCK_TIME).format("d[D] h[H] m[M] s[S]");
}
