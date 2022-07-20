import _ from "lodash";
import moment from "moment";
import { ChartRange } from "./types";
/**
 * 
 * @param ms 
 * @param limit 
 * @param valueMin 
 * @param valueMax 
 * @returns 
 */
export function generateRandomSubsquidTvlData(
  ms: number,
  limit: number = 100,
  valueMin: number = 1000,
  valueMax: number = 5000
): [number, number][] {
  const max = Date.now();
  const min = max - ms;
  let data: [number, number][] = [];

  for (let i = 0; i < limit; i++) {
    const randomInRangeTs = Math.floor(_.random(min, max));
    const value = _.random(valueMin, valueMax);

    data.push([randomInRangeTs, value]);
  }

  return data.sort((a, b) => {
    return b[0] - a[0];
  });
}
/**
 * 
 * @param timestamp 
 * @param rangeLimit 
 * @param chartInterval 
 * @returns 
 */
export function getSelectedChartRangeLimitTimestamp(
  timestamp: number,
  rangeLimit: "start" | "end",
  chartInterval: ChartRange
): number {
  switch (chartInterval) {
    case "24h":
      return rangeLimit == "start"
        ? moment(timestamp).startOf("h").valueOf()
        : moment(timestamp).endOf("h").valueOf();
    case "1w":
      return rangeLimit == "start"
        ? moment(timestamp).startOf("week").valueOf()
        : moment(timestamp).endOf("week").valueOf();
    case "1m":
      return rangeLimit == "start"
        ? moment(timestamp).startOf("month").valueOf()
        : moment(timestamp).endOf("month").valueOf();
    default:
      return timestamp;
  }
}
/**
 * Provides next sequence timestamps
 * given a sequence timestamp
 * @param rangeTs number (timestamp)
 * @param range ChartRange "24h" | "1w" | "1m"
 * @returns [number, number]
 */
export function getNextRangeGivenTimestamp(
  rangeTs: number,
  range: ChartRange
): [number, number] {
  switch (range) {
    case "24h":
      let nextHourRange = moment(rangeTs).add(1, "hour");
      return [nextHourRange.valueOf(), nextHourRange.endOf("hour").valueOf()];
    case "1w":
      let nextWeekRange = moment(rangeTs).add(1, "week");
      return [nextWeekRange.valueOf(), nextWeekRange.endOf("week").valueOf()];
    case "1m":
      let nextMonthRange = moment(rangeTs).add(1, "month");
      return [
        nextMonthRange.valueOf(),
        nextMonthRange.endOf("month").valueOf(),
      ];
  }
}
