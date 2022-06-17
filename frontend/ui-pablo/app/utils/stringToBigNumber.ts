import BigNumber from "bignumber.js";

export const stringToBigNumber = (value: string) => {
  try {
    return new BigNumber(value.replaceAll(",", ""));
  } catch (err) {
    return new BigNumber(value);
  }
};
