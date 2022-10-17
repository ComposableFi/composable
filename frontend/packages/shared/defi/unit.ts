import BigNumber from "bignumber.js";
import { PERBILL_UNIT, PERMILL_UNIT } from "./constants";

export function toChainIdUnit(value: number | BigNumber, decimalPlaces = 12) {
  const bigNumberValue =
    typeof value === "number" ? new BigNumber(value) : value;

  return bigNumberValue.multipliedBy(10 ** decimalPlaces);
}

export function fromChainIdUnit(value: number | BigNumber | BigInt, decimalPlaces = 12) {
  return (BigNumber.isBigNumber(value) ? value : new BigNumber(value.toString())).dividedBy(
    10 ** decimalPlaces
  );
}

export function fromPermill(
  amount: string | number
): BigNumber {
  return new BigNumber(amount).div(PERMILL_UNIT).times(100);
}

export function fromPerbill(
  amount: string | number
): BigNumber {
  return new BigNumber(amount).div(PERBILL_UNIT).times(100);
}
