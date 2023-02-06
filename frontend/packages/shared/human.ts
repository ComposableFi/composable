import { secondsToDhms } from "shared";
import BigNumber from "bignumber.js";

export function humanBalance(balance: string | number | BigNumber) {
  const formatter = new Intl.NumberFormat("en", {
    notation: "compact",
  });

  return formatter.format(Number(balance.toString()));
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

export function humanDateDiff(date: Date) {
  const diff: number = date.getTime() - new Date().getTime();
  const formatter = new Intl.RelativeTimeFormat("en");

  return formatter.format(Math.ceil(diff / 86400_000), "days");
}
