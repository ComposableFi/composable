import { fetchAuctionChartSeries } from "@/defi/utils/pablo/auctions";
import { useEffect, useRef } from "react";
import { PabloLiquidityBootstrappingPool } from "shared";

export function useAuctionsChart(
  pool: PabloLiquidityBootstrappingPool | null
): {
  currentPriceSeries: [number, number][];
  predictedPriceSeries: [number, number][];
} {
  let auctionChartSeries = useRef({
    currentPriceSeries: [] as [number, number][],
    predictedPriceSeries: [] as [number, number][],
  });

  useEffect(() => {
    fetchAuctionChartSeries(pool).then((response) => {
      auctionChartSeries.current.currentPriceSeries = response.chartSeries
      auctionChartSeries.current.predictedPriceSeries = response.predictedSeries
    });
  }, [pool]);

  const { currentPriceSeries, predictedPriceSeries } = auctionChartSeries.current;

  return {
    currentPriceSeries,
    predictedPriceSeries,
  };
}