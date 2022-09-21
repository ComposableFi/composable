import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { fetchAuctionChartSeries } from "@/defi/utils/pablo/auctions";
import { useEffect, useState } from "react";
import { useParachainApi } from "substrate-react";
import { LiquidityBootstrappingPool } from "@/defi/types";
import { queryPabloTransactions } from "@/defi/subsquid/pools/queries";

export function useAuctionsChart(
  pool: LiquidityBootstrappingPool | undefined
): {
  currentPriceSeries: [number, number][];
  predictedPriceSeries: [number, number][];
} {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  let [auctionChartSeries, setAuctionChartSeries] = useState<{
    currentPriceSeries: [number, number][];
    predictedPriceSeries: [number, number][];
  }>({
    currentPriceSeries: [],
    predictedPriceSeries: [],
  });

  useEffect(() => {
    if (pool && parachainApi) {
      fetchAuctionChartSeries(parachainApi, pool).then((response) => {
        setAuctionChartSeries({
          currentPriceSeries: response.chartSeries,
          predictedPriceSeries: response.predictedSeries,
        });
      });
    } else {
      setAuctionChartSeries({
        currentPriceSeries: [],
        predictedPriceSeries: [],
      });
    }
  }, [pool, parachainApi]);

  const { currentPriceSeries, predictedPriceSeries } = auctionChartSeries;

  return {
    currentPriceSeries,
    predictedPriceSeries,
  };
}