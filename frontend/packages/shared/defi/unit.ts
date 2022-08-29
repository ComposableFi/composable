import BigNumber from "bignumber.js";

export function toChainIdUnit(value: number | BigNumber, decimalPlaces = 12) {
  const bigNumberValue =
    typeof value === "number" ? new BigNumber(value) : value;

  return bigNumberValue.multipliedBy(10 ** decimalPlaces);
}

export function fromChainIdUnit(value: number | BigNumber, decimalPlaces = 12) {
  return (typeof value === "number" ? new BigNumber(value) : value).dividedBy(
    10 ** decimalPlaces
  );
}
