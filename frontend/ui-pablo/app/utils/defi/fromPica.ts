import BigNumber from "bignumber.js";
import { DEFAULT_DECIMALS } from "../constants";

export function fromPica(value: BigNumber) {
  return value.div(DEFAULT_DECIMALS);
}
