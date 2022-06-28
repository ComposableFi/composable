import BigNumber from "bignumber.js";

export function fromChainUnits(
  amount: string | number,
  decimals: number = 12
): BigNumber {
  const base = new BigNumber(10);
  return new BigNumber(amount).div(base.pow(decimals));
}

export function toChainUnits(
  amount: string | number | BigNumber,
  decimals: number = 12
): BigNumber {
  const base = new BigNumber(10);
  return new BigNumber(amount).times(base.pow(decimals));
}
