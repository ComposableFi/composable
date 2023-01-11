import BigNumber from "bignumber.js";
import { PERBILL_UNIT, PERMILL_UNIT } from "./constants";

export function fromChainUnits(
  amount: string | number | BigNumber,
  decimals: number = 12
): BigNumber {
  const base = new BigNumber(10);
  return new BigNumber(amount).div(base.pow(decimals));
}

export function toChainUnits(
  amount: string | number | BigNumber,
  decimals: number = 12
): BigNumber {
  const BN = BigNumber.clone({
    EXPONENTIAL_AT: 999,
    DECIMAL_PLACES: decimals,
  });
  const base = BN(10);
  return BN(amount).times(base.pow(decimals)).dp(0);
}

export function fromPermill(amount: string | number): BigNumber {
  return new BigNumber(amount).div(PERMILL_UNIT).times(100);
}

export function fromPerbill(amount: string | number): BigNumber {
  return new BigNumber(amount).div(PERBILL_UNIT);
}
