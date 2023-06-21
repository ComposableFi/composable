import { ChartRange } from "./index";
import { getSelectedChartRangeLimitTimestamp, getNextRangeGivenTimestamp } from "./utils";
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
     * the redundant data in subsquid response
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
