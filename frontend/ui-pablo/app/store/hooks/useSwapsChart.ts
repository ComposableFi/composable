import { DEFI_CONFIG } from "@/defi/config";
import { useState, useEffect } from "react";
import BigNumber from "bignumber.js";
import useStore from "../useStore";
import { ChartRange } from "@/defi/utils/charts";
import { fetch24HourOldPrice, fetchSwapsChart } from "@/defi/subsquid/queries/swaps";

export const useSwapsChart = () => {
  const { swaps } = useStore();
  const { selectedAssets, selectedPool } = swaps;
  const { quote } = selectedAssets;
  const [seriesIntervals, setSeriesIntervals] = useState<string[]>([]);
  const [chartSeries, setChartSeries] = useState<[number, number][]>([]);
  const [selectedInterval, setSelectedInterval] = useState(
    DEFI_CONFIG.swapChartIntervals[0]
  );
  const [_24hourOldPrice, set24HourOldPrice] = useState(new BigNumber(0));

  useEffect(() => {
    if (selectedPool && selectedPool.poolId !== -1 && quote !== "none") {
      fetchSwapsChart(
        selectedPool.poolId,
        quote,
        selectedInterval.symbol as ChartRange
      ).then((series) => {
        setChartSeries(series);
      });
    }
  }, [selectedPool, quote, selectedInterval]);

  useEffect(() => {
    if (selectedPool && selectedPool.poolId !== -1 && quote !== "none") {
      fetch24HourOldPrice(selectedPool.poolId, quote).then((oldPrice) => {
        set24HourOldPrice(oldPrice);
      });
    }
  }, [selectedPool, quote]);

  return {
    selectedInterval,
    setSelectedInterval,
    chartSeries,
    seriesIntervals,
    _24hourOldPrice,
  };
};
