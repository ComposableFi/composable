import { DEFI_CONFIG } from "@/defi/config";
import { ChartInterval } from "@/defi/types";
import { queryLiquidityByPoolId } from "@/updaters/liquidity/subsquid";
import BigNumber from "bignumber.js";
import moment from "moment";
import { useState, useEffect } from "react";
import { usePoolDetails } from "./usePoolDetails";

const HOUR = 1 * 60 * 60 * 1000;
const DAY = 24 * HOUR;

function aggragateData(
  data: [number, number][],
  chartInterval: "24h" | "1w" | "1m"
): [number, number][] {
  const processed: [number, number][] = [];
  if (!data.length) return processed;

  switch (chartInterval) {
    case "24h":
      const yesterday = data[0][0] - DAY;

      const toAggregate = data.filter((i: [number, number]) => {
        return i[0] > yesterday;
      });

      let startTs = moment(data[0][0]).startOf("h").valueOf();

      while (startTs > yesterday) {
        let dataPointStart = startTs;
        const dataPointEnd = startTs + HOUR;

        let maxTvlForThisPoint = toAggregate.reduce((p, c) => {
          if (c[0] > dataPointStart - HOUR && c[0] < dataPointStart) {
            return c[1];
          }
          return p;
        }, 0);

        let thisPointTvl = 0;
        toAggregate.forEach((item) => {
          if (
            item[0] > dataPointStart &&
            item[0] < dataPointEnd &&
            item[1] > thisPointTvl
          ) {
            thisPointTvl = item[1];
          }
        });

        if (thisPointTvl !== 0) {
          maxTvlForThisPoint = thisPointTvl;
        }

        processed.push([dataPointStart, maxTvlForThisPoint]);
        startTs = startTs - HOUR;
      }

      return processed;
    case "1w":
    case "1m":
      let key = chartInterval == "1m" ? "month" : "week";
      let startOf = moment(data[0][0])
        .startOf(key as any)
        .valueOf();

      while (startOf > data[data.length - 1][0]) {
        const endOf = moment(startOf)
          .endOf(key as any)
          .valueOf();

        let lastStartOf =
          chartInterval === "1w"
            ? startOf - 7 * DAY
            : (startOf = moment(startOf - HOUR)
                .startOf(key as any)
                .valueOf());

        let maxTvlForThisPoint = data.reduce((p, c) => {
          if (c[0] > lastStartOf && c[0] < startOf) {
            return c[1];
          }
          return p;
        }, 0);

        let thisPointTvl = 0;
        data.forEach((item) => {
          if (item[0] > startOf && item[0] < endOf && item[1] > thisPointTvl) {
            thisPointTvl = item[1];
          }
        });

        if (thisPointTvl !== 0) {
          maxTvlForThisPoint = thisPointTvl;
        }

        processed.push([startOf, maxTvlForThisPoint]);

        if (chartInterval === "1w") {
          startOf = startOf - 7 * DAY;
        } else {
          startOf = moment(startOf - HOUR)
            .startOf(key as any)
            .valueOf();
        }
      }
    default:
      return processed;
  }
}

function createIntervals (series: [number, number][], chartInterval: ChartInterval): string[] {
  let intervals: string[] = [];
  if (!series.length) return intervals;

  switch(chartInterval) {
    case "24h":
      for (let i = 0; i < series.length; i += 6) {
        if (series[i]) {
          intervals.push(moment(series[i][0]).format("hh:mm"))
        }
      }
      return intervals
    default:
      return intervals
  }

}

export const usePoolTvlChart = (poolId: number) => {
  const poolDetails = usePoolDetails(poolId);

  const [seriesIntervals, setSeriesIntervals] = useState<string[]>([])
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

          const series = aggragateData(data, selectedInterval.symbol as any);
          setChartSeries(series);
          setSeriesIntervals(createIntervals(series, selectedInterval.symbol as any))
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
    seriesIntervals
  };
};
