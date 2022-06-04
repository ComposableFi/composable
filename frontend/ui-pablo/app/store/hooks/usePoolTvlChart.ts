import { DEFI_CONFIG } from "@/defi/config";
import { ChartInterval } from "@/defi/types";
import { queryLiquidityByPoolId } from "@/updaters/liquidity/subsquid";
import BigNumber from "bignumber.js";
import moment from "moment";
import { useState, useEffect } from "react";
import { usePoolDetails } from "./usePoolDetails";

function getIntervalRangeLimit(
  timestamp: number,
  rangeLimit: "start" | "end",
  chartInterval: "24h" | "1w" | "1m"
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
    case "1w":
      return rangeLimit == "start"
        ? moment(timestamp).startOf("month").valueOf()
        : moment(timestamp).endOf("month").valueOf();
    default:
      return timestamp;
  }
}

function processTvlChartSeries(
  data: [number, number][],
  chartInterval: "24h" | "1w" | "1m"
): [number, number][] {
  if (!data.length) return data;

  let rangeStart: Set<number> | Array<number> = new Set(
    data.map((i) => getIntervalRangeLimit(i[0], "start", chartInterval))
  );
  let rangeEnd: Set<number> | Array<number> = new Set(
    data.map((i) => getIntervalRangeLimit(i[0], "end", chartInterval))
  );

  rangeStart = Array.from(rangeStart).sort((a, b) => {
    return b - a;
  });
  rangeEnd = Array.from(rangeStart).sort((a, b) => {
    return b - a;
  });

  const intervals = rangeStart.reduce((p, c, i) => {
    p.push([c, (rangeEnd as number[])[i]])
    return p;
  }, [] as [number, number][])

  console.log(intervals)

  return [];
}

function createIntervals(
  series: [number, number][],
  chartInterval: ChartInterval
): string[] {
  let intervals: string[] = [];
  if (!series.length) return intervals;

  switch (chartInterval) {
    case "24h":
      for (let i = 0; i < series.length; i += 6) {
        if (series[i]) {
          intervals.push(moment(series[i][0]).format("hh:mm"));
        }
      }
      return intervals;
    default:
      return intervals;
  }
}

export const usePoolTvlChart = (poolId: number) => {
  const poolDetails = usePoolDetails(poolId);

  const [seriesIntervals, setSeriesIntervals] = useState<string[]>([]);
  const [chartSeries, setChartSeries] = useState<[number, number][]>([]);
  const [selectedInterval, setSelectedInterval] = useState(
    DEFI_CONFIG.swapChartIntervals[0]
  );

  useEffect(() => {
    if (poolId !== -1 && poolDetails.quoteAsset) {
      const quoteDecs = new BigNumber(10).pow(poolDetails.quoteAsset.decimals);
      queryLiquidityByPoolId(poolId)
        .then((response) => {
          const { pabloPools } = response.data;

          const data = pabloPools.map((i: any) => {
            return [
              Number(i.calculatedTimestamp),
              new BigNumber(i.totalLiquidity).div(quoteDecs).toNumber(),
            ];
          });

          const series = processTvlChartSeries(
            data,
            selectedInterval.symbol as any
          );
          setChartSeries(series);
          setSeriesIntervals(
            createIntervals(series, selectedInterval.symbol as any)
          );
        })
        .catch((err) => {
          console.log("Error fetching chart data", err.message);
          setChartSeries([]);
        });
    } else {
      setChartSeries([]);
    }
  }, [poolId, selectedInterval]);

  return {
    selectedInterval,
    setSelectedInterval,
    chartSeries,
    seriesIntervals,
  };
};
