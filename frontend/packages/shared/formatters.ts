import { TokenId, TOKENS } from "tokens";
import BigNumber from "bignumber.js";

export const formatToken = (amount: BigNumber | number, tokenId: TokenId) => {
  return `${amount} ${TOKENS[tokenId].symbol}`;
};

// add commas to large numbers
export const formatNumber = (amount: BigNumber | number) => {
  if (amount instanceof BigNumber) {
    return amount.toFormat(0);
  }
  return amount.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
};

export const dateFromNumber = (timestamp: number | BigInt) => {
  return new Date(Number(timestamp.toString()) * 1000);
};

export const formatDate = (date: Date) => {
  return date.toLocaleDateString();
};

export const formatNumberWithSymbol = (
  amount: BigNumber | number,
  symbol: string,
  symbolEnd?: string
) => {
  let formatted = formatNumber(amount);
  // Add symbol after minus sign
  return formatted.startsWith("-") && symbolEnd
    ? formatted.substring(0, 1) + symbol + formatted.substring(1) + symbolEnd
    : formatted.startsWith("-")
    ? formatted.substring(0, 1) + symbol + formatted.substring(1)
    : symbolEnd
    ? symbol + formatted + symbolEnd
    : symbol + formatted;
};

export const formatNumberCompact = (amount: number) => {
  return `${Intl.NumberFormat("en", {
    notation: "compact",
    minimumFractionDigits: 0,
    maximumFractionDigits: 2,
  }).format(amount)}`;
};

export const formatNumberCompactWithToken = (
  amount: number,
  tokenId: TokenId
) => {
  return `${Intl.NumberFormat("en", {
    notation: "compact",
    minimumFractionDigits: 0,
    maximumFractionDigits: 2,
  }).format(amount)} ${TOKENS[tokenId].symbol}`;
};

export const formatNumberCompactWithSymbol = (
  amount: number,
  symbol: string
) => {
  return `${symbol}${Intl.NumberFormat("en", {
    notation: "compact",
    minimumFractionDigits: 0,
  }).format(amount)}`;
};
