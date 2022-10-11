import { fetchAuctionChartSeries } from "@/defi/utils/pablo/auctions";
import { useEffect, useRef } from "react";
import { LiquidityBootstrappingPool } from "@/defi/types";
import { ApiPromise } from "@polkadot/api";

export function useAuctionsChart(
  api?: ApiPromise,
  pool?: LiquidityBootstrappingPool
): {
  currentPriceSeries: [number, number][];
  predictedPriceSeries: [number, number][];
} {
  let auctionChartSeries = useRef({
    currentPriceSeries: [] as [number, number][],
    predictedPriceSeries: [] as [number, number][],
  });

  useEffect(() => {
    fetchAuctionChartSeries(api, pool).then((response) => {
      auctionChartSeries.current.currentPriceSeries = response.chartSeries
      auctionChartSeries.current.predictedPriceSeries = response.predictedSeries
    });
  }, [pool, api]);

  const { currentPriceSeries, predictedPriceSeries } = auctionChartSeries.current;

  return {
    currentPriceSeries,
    predictedPriceSeries,
  };
}