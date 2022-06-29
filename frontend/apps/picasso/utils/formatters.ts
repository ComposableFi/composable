import { TokenId, TOKENS } from "@/defi/Tokens";
import BigNumber from "bignumber.js";
import { secondsToDHMS } from "@/defi/polkadot/hooks/useBondVestingInDays";

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

export const dateFromNumber = (timestamp: number) => {
  return new Date(timestamp * 1000);
};

export const formatDate = (date: Date) => {
  return `${date.getDate().toString().padStart(2, "0")}/${(date.getMonth() + 1)
    .toString()
    .padStart(2, "0")}/${date.getFullYear()}`;
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

export function humanBalance(balance: string | number | BigNumber) {
  const THOUSAND = 1000;
  const MILLION = 1000_000;
  const BILLION = 1000_000_000;
  const TRILLION = 1000_000_000_000;
  const QUADRILLION = 1000_000_000_000_000;
  const QUINTILLION = 1000_000_000_000_000_000;

  const VALUES = [
    {
      unit: "K",
      size: THOUSAND,
    },
    {
      unit: "M",
      size: MILLION,
    },

    {
      unit: "G",
      size: BILLION,
    },
    {
      unit: "T",
      size: TRILLION,
    },
    {
      unit: "P",
      size: QUADRILLION,
    },
    {
      unit: "E",
      size: QUINTILLION,
    },
  ];

  type ReducerReturnValue = {
    unit: string;
    amount: string;
  };
  const newValue =
    typeof balance === "number" || typeof balance === "string"
      ? new BigNumber(balance)
      : balance;

  const trailingZeros = /^0*(\d+(?:\.(?:(?!0+$)\d)+)?)/;
  const out = VALUES.reduce((acc, { unit, size }) => {
    if (newValue.gte(new BigNumber(size))) {
      acc = {
        unit,
        amount: newValue.div(size).toFixed(),
      };
    } else {
      acc = {
        unit: "",
        amount: newValue.toFixed(),
      };
    }

    return acc;
  }, <ReducerReturnValue>{});

  const match = trailingZeros.exec(out.amount);
  if (match !== null) {
    return out.unit + match[1];
  }
  return out.unit + out.amount;
}

export const SHORT_HUMAN_DATE = 1;
export const LONG_HUMAN_DATE = 2;

export function humanDate(date: number, option: number = SHORT_HUMAN_DATE) {
  const toDHMS = secondsToDHMS(date);

  if (option === SHORT_HUMAN_DATE) {
    const output = [
      toDHMS.d === 0 ? "" : toDHMS.d.toString().padStart(2, "0") + "D",
      toDHMS.h === 0 ? "" : toDHMS.h.toString().padStart(2, "0") + "H",
      toDHMS.m === 0 ? "" : toDHMS.m.toString().padStart(2, "0") + "M",
      toDHMS.s === 0 ? "" : toDHMS.s.toString().padStart(2, "0") + "S",
    ].join("");

    return output.length > 0 ? output : "~";
  }

  return [
    toDHMS.d === 0 ? "" : toDHMS.dDisplay,
    toDHMS.h === 0 ? "" : toDHMS.hDisplay,
    toDHMS.m === 0 ? "" : toDHMS.mDisplay,
    toDHMS.s === 0 ? "" : toDHMS.sDisplay,
  ].join(" ");
}
