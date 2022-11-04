import { DEFI_CONFIG } from "@/defi/config";
import { queryLiquidityByPoolId } from "@/defi/subsquid/liquidity/queries";
import { useState, useEffect } from "react";
import { processSubsquidChartData } from "@/defi/utils/charts";
import BigNumber from "bignumber.js";
import { useLiquidityPoolDetails } from "./useLiquidityPoolDetails";

export const usePoolTvlChart = (poolId: number) => {
  const poolDetails = useLiquidityPoolDetails(poolId);

  const [seriesIntervals, setSeriesIntervals] = useState<string[]>([]);
  const [chartSeries, setChartSeries] = useState<[number, number][]>([]);
  const [selectedInterval, setSelectedInterval] = useState(
    DEFI_CONFIG.swapChartIntervals[0]
  );

  useEffect(() => {
    if (poolId !== -1 && poolDetails.quoteAsset) {
      const quoteDecs = new BigNumber(10).pow(poolDetails.quoteAsset.getDecimals());
      queryLiquidityByPoolId(poolId)
        .then((response) => {
          const { pabloPools } = response.data;

          const data = pabloPools.map((i: any) => {
            return [
              Number(i.calculatedTimestamp),
              new BigNumber(i.totalLiquidity).div(quoteDecs).toNumber(),
            ];
          });

          const series = processSubsquidChartData(
            data,
            selectedInterval.symbol as any
          );
          setChartSeries(series);
        })
        .catch((err) => {
          console.log("Error fetching chart data", err.message);
          setChartSeries([]);
        });
    } else {
      setChartSeries([]);
    }
  }, [poolId, poolDetails.quoteAsset, selectedInterval]);

  return {
    selectedInterval,
    setSelectedInterval,
    chartSeries,
    seriesIntervals,
  };
};