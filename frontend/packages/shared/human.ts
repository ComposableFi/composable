import { secondsToDhms } from "shared";
import BigNumber from "bignumber.js";

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
  const out = VALUES.reduce(
    (acc, { unit, size }) => {
      if (newValue.gte(new BigNumber(size))) {
        acc = {
          unit,
          amount: newValue.div(size).toFixed(),
        };
      }

      return acc;
    },
    <ReducerReturnValue>{
      unit: "",
      amount: newValue.toFixed(),
    }
  );

  const match = trailingZeros.exec(out.amount);

  return match !== null ? match[1] + out.unit : out.amount + out.unit;
}

export const SHORT_HUMAN_DATE = 1;
export const LONG_HUMAN_DATE = 2;

export function humanDate(date: number, option: number = SHORT_HUMAN_DATE) {
  const toDHMS = secondsToDhms(date);

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

export function humanDateDiff(date1: Date, date2: Date) {
  const diff: number = date2.getTime() - date1.getTime();
  const SECONDS_IN_DAY = 86400;
  const SECONDS_IN_WEEK = SECONDS_IN_DAY * 7;
  const SECONDS_IN_MONTH = SECONDS_IN_DAY * 30;
  const SECONDS_IN_YEAR = SECONDS_IN_MONTH * 12;

  const target = diff / 1000; // calculate in seconds;

  if (target >= SECONDS_IN_YEAR) {
    const result = Math.trunc(target / SECONDS_IN_YEAR);
    return [result, `year${result > 1 ? "s" : ""}`];
  } else if (target >= SECONDS_IN_MONTH) {
    const result = Math.trunc(target / SECONDS_IN_MONTH);
    return [result, `month${result > 1 ? "s" : ""}`];
  } else if (target >= SECONDS_IN_WEEK) {
    const result = Math.trunc(target / SECONDS_IN_WEEK);
    return [result, `week${result > 1 ? "s" : ""}`];
  } else if (target >= SECONDS_IN_DAY) {
    const result = Math.trunc(target / SECONDS_IN_DAY);
    return [result, `day${result > 1 ? "s" : ""}`];
  } else {
    return [0, "day"];
  }
}
