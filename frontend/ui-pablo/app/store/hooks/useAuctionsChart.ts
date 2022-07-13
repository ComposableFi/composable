import { LiquidityBootstrappingPoolTrade } from "@/defi/types/auctions";
import {
  createPabloPoolAccountId,
  DEFAULT_NETWORK_ID,
  fetchBalanceByAssetId,
} from "@/defi/utils";
import { calculatePredictedChartSeries } from "@/defi/utils/charts/auctions";
import { fetchAuctionChartSeries, transformAuctionsTransaction } from "@/defi/utils/pablo/auctions";

import { queryPoolTransactionsByType } from "@/subsquid/queries/pools";
import BigNumber from "bignumber.js";
import { useEffect, useState } from "react";
import { useParachainApi } from "substrate-react";
import { LiquidityBootstrappingPool } from "@/defi/types";
import moment from "moment";

export function useAuctionsChart(
  pool: LiquidityBootstrappingPool | undefined
): {
  currentPriceSeries: [number, number][];
  predictedPriceSeries: [number, number][];
} {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  let [currentPriceSeries, setCurrentPriceSeries] = useState<
    [number, number][]
  >([]);
  let [predictedPriceSeries, setPredictedPriceSeries] = useState<
    [number, number][]
  >([]);

  useEffect(() => {
    if (pool && parachainApi) {
      fetchAuctionChartSeries(parachainApi, pool).then(response => {
        setCurrentPriceSeries(response.chartSeries);
        setPredictedPriceSeries(response.predictedSeries);
      })
    } else {
      setCurrentPriceSeries([]);
      setPredictedPriceSeries([]);
    }
  }, [pool, parachainApi]);

  return {
    currentPriceSeries,
    predictedPriceSeries,
  };
}
