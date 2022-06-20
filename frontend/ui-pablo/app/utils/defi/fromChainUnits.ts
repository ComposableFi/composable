import BigNumber from "bignumber.js";
import { DEFAULT_DECIMALS } from "../constants";

export function fromChainUnits(value: BigNumber) {
  return value.div(DEFAULT_DECIMALS);
}
