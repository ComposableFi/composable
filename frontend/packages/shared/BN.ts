import BigNumber from "bignumber.js";
import BN from "bn.js";
/**
 * Convert 10.999 to 10999000
 */
export const toBaseUnitBN = (
  rawAmt: string | number | BigNumber,
  decimals: number
) => {
  const raw = new BigNumber(rawAmt);
  const base = new BigNumber(10);
  const decimalsBN = new BigNumber(decimals);
  return raw.times(base.pow(decimalsBN)).toNumber();
};

/**
 * Convert 10999000 to 10.999
 */
export const toTokenUnitsBN = (
  tokenAmount: string | number | BN,
  tokenDecimals: number
) => {
  const amt = new BigNumber(tokenAmount.toString());
  const digits = new BigNumber(10).pow(new BigNumber(tokenDecimals));
  return amt.div(digits);
};

