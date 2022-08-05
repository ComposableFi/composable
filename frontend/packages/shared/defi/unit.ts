import BigNumber from "bignumber.js";

export function toChainIdUnit(value: number | BigNumber) {
  const bigNumberValue =
    typeof value === "number" ? new BigNumber(value) : value;

  return bigNumberValue.multipliedBy(10 ** 12);
}

export function fromChainIdUnit(value: number | BigNumber) {
  return (typeof value === "number" ? new BigNumber(value) : value).dividedBy(
    10 ** 12
  );
}
