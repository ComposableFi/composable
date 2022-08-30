import BigNumber from "bignumber.js";

export const FLOAT_NUMBER: RegExp = /^\d+(\.\d+)?$/;
export const NUMBERS_ONE_DOT: RegExp = /^\d+\.$/;

export const nFormatter = (num: number, digits: number = 1) => {
  const lookup = [
    { value: 1, symbol: "" },
    { value: 1e3, symbol: "k" },
    { value: 1e6, symbol: "M" },
    { value: 1e9, symbol: "B" },
    { value: 1e12, symbol: "T" },
    { value: 1e15, symbol: "P" },
    { value: 1e18, symbol: "E" }
  ];
  const rx = /\.0+$|(\.[0-9]*[1-9])0+$/;
  const item = lookup.slice().reverse().find(function(item) {
    return num >= item.value;
  });
  return item ? (num / item.value).toFixed(digits).replace(rx, "$1") + item.symbol : "0";
}


export const validNumber = (value: number | string, min?: number, max?: number) => {
  const numberValue = Number(value);
  return !isNaN(numberValue)
            && !(min && numberValue < min)
            && !(max && numberValue > max);
}

export const humanizedBnToBn = (bn: string | number): BigNumber => {
  return new BigNumber(typeof bn === "string" ? bn.replaceAll(",", "") : bn)
};
export const humanizedPermillToBigNumber = (permill: string) => new BigNumber(permill.replaceAll("%", ""));
