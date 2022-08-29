import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { fetchAuctionChartSeries } from "@/defi/utils/pablo/auctions";
import { useEffect, useState } from "react";
import { useParachainApi } from "substrate-react";
import { LiquidityBootstrappingPool } from "@/defi/types";

export function useAuctionsChart(
  pool: LiquidityBootstrappingPool | undefined
): {
  currentPriceSeries: [number, number][];
  predictedPriceSeries: [number, number][];
} {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  let [auctionChartSerie, setAuctionChartSerie] = useState<{
    currentPriceSeries: [number, number][];
    predictedPriceSeries: [number, number][];
  }>({
    currentPriceSeries: [],
    predictedPriceSeries: [],
  });

  useEffect(() => {
    if (pool && parachainApi) {
      fetchAuctionChartSeries(parachainApi, pool).then((response) => {
        setAuctionChartSerie({
          currentPriceSeries: response.chartSeries,
          predictedPriceSeries: response.predictedSeries,
        });
      });
    } else {
      setAuctionChartSerie({
        currentPriceSeries: [],
        predictedPriceSeries: [],
      });
    }
  }, [pool, parachainApi]);

  const { currentPriceSeries, predictedPriceSeries } = auctionChartSerie;

  return {
    currentPriceSeries,
    predictedPriceSeries,
  };
}