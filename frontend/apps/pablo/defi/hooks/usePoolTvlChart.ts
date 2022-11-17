import { DEFI_CONFIG } from "@/defi/config";
import { queryLiquidityByPoolId } from "@/defi/subsquid/liquidity/queries";
import { useState, useEffect } from "react";
import { processSubsquidChartData } from "@/defi/utils/charts";
import { useLiquidityPoolDetails } from "./useLiquidityPoolDetails";
import { fromChainIdUnit } from "shared";

export const usePoolTvlChart = (poolId: number) => {
  const poolDetails = useLiquidityPoolDetails(poolId);

  const [seriesIntervals, setSeriesIntervals] = useState<string[]>([]);
  const [chartSeries, setChartSeries] = useState<[number, number][]>([]);
  const [selectedInterval, setSelectedInterval] = useState(
    DEFI_CONFIG.swapChartIntervals[0]
  );

  useEffect(() => {
    const quoteDecimals = poolDetails.quoteAsset?.getDecimals("picasso");
    if (poolId !== -1 && poolDetails.quoteAsset && quoteDecimals) {
      queryLiquidityByPoolId(poolId)
        .then((response) => {
          const { pabloPools } = response.data;

          const data = pabloPools.map((i: any) => {
            return [
              Number(i.calculatedTimestamp),
              fromChainIdUnit(BigInt(i.totalLiquidity), quoteDecimals),
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