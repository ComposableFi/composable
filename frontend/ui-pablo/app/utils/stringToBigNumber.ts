import BigNumber from "bignumber.js";

export const stringToBigNumber = (value: string) =>
  new BigNumber(value.replaceAll(",", ""));
