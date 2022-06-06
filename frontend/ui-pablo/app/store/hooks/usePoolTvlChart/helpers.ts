import _ from "lodash";
import moment from "moment";

export type ChartRange = "24h" | "1w" | "1m";

export const generateRandomSubsquidTvlData = (
  ms: number,
  limit: number = 100,
  valueMin: number = 1000,
  valueMax: number = 5000
): [number, number][] => {
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

function getSelectedChartRangeLimitTimestamp(
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
function getNextRangeGivenTimestamp(rangeTs: number, range: ChartRange): [number, number] {
  switch(range) {
    case "24h":
      let nextHourRange = moment(rangeTs).add(1, 'hour');
      return [nextHourRange.valueOf(), nextHourRange.endOf('hour').valueOf()]
    case "1w":
      let nextWeekRange = moment(rangeTs).add(1, 'week')
      return [nextWeekRange.valueOf(), nextWeekRange.endOf('week').valueOf()]
    case "1m":
      let nextMonthRange = moment(rangeTs).add(1, 'month')
      return [nextMonthRange.valueOf(), nextMonthRange.endOf('month').valueOf()]
  }
}
/**
 * This method processes subsquid data
 * into a representable chart series
 * Expects data in a DESC order format
 * w.r.t to calculatedTimestamp property
 * from subsquid
 * @param data is an array of number array ([timestamp, value][])
 * @param range ChartRange "24h" | "1w" | "1m"
 * @returns number array ([timestamp, value])
 */
export const processSubsquidChartData = (
  data: [number, number][],
  range: ChartRange
): [number, number][] => {
  if (!data.length) return data;
  /**
   * Creating an array from set to filter out
   * the retundant data in subsquid response
   * as we only care about the most recent
   * tvl within a timeframe/selected range
   */
  let rangeStart = Array.from(
    new Set(data.map((i) => getSelectedChartRangeLimitTimestamp(i[0], "start", range)))
  );
  let rangeEnd = Array.from(
    new Set(data.map((i) => getSelectedChartRangeLimitTimestamp(i[0], "end", range)))
  );
  /**
   * Subsquid provided data in such a form
   * that its possible for it to miss a sequence
   * of timestamps, therefore, FE adds missing
   * ranges itself and sets the value to previous
   * frame's recent value, if found
   */
  let withMissingIntervals: [number, number][] = [];
  for (let i = rangeStart.length - 1; i >= 0; i--) {

    let totalValueLocked = 0;

    for (let tvlIndex = data.length - 1; tvlIndex >= 0; tvlIndex--) {
      if (data[tvlIndex][0] > rangeStart[i] && data[tvlIndex][0] < rangeEnd[i]) {
        totalValueLocked = data[tvlIndex][1]; // care about the latest index only
      }
    }

    withMissingIntervals.push([rangeStart[i], totalValueLocked])
    let nextRange = getNextRangeGivenTimestamp(rangeStart[i], range)

    while (nextRange[0] !== rangeStart[i - 1] && i > 0) {
      withMissingIntervals.push([nextRange[0], totalValueLocked])
      nextRange = getNextRangeGivenTimestamp(nextRange[0], range)
    }
  }
  
  return withMissingIntervals;
}