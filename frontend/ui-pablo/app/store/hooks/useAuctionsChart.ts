import { LiquidityBootstrappingPoolTrade } from "@/defi/types/auctions";
import {
  createPabloPoolAccountId,
  DEFAULT_NETWORK_ID,
  fetchBalanceByAssetId,
} from "@/defi/utils";
import { calculatePredictedChartSeries } from "@/defi/utils/charts/auctions";
import { transformAuctionsTransaction } from "@/defi/utils/pablo/auctions";

import { queryPoolTransactionsByType } from "@/updaters/pools/subsquid";
import BigNumber from "bignumber.js";
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
  let [currentPriceSeries, setCurrentPriceSeries] = useState<
    [number, number][]
  >([]);
  let [predictedPriceSeries, setPredictedPriceSeries] = useState<
    [number, number][]
  >([]);

  useEffect(() => {
    if (pool && parachainApi) {
      queryPoolTransactionsByType(pool.poolId, "SWAP").then((res) => {
        let swapTxs: LiquidityBootstrappingPoolTrade[] = [];
        if ((res as any).data && (res as any).data.pabloTransactions) {
          swapTxs = (res as any).data.pabloTransactions.map((t: any) =>
            transformAuctionsTransaction(t, pool.pair.quote)
          );

          let currentPrices = swapTxs.map((i) => {
            return [i.receivedTimestamp, Number(i.spotPrice)];
          }) as [number, number][];

          setCurrentPriceSeries(currentPrices);

          if (currentPrices.length) {
            parachainApi.query.system.number().then(async (blockNo) => {
              const poolAccount = createPabloPoolAccountId(
                parachainApi,
                pool.poolId
              );
              const quoteBal = await fetchBalanceByAssetId(
                parachainApi,
                poolAccount,
                pool.pair.quote.toString()
              );
              const baseBal = await fetchBalanceByAssetId(
                parachainApi,
                poolAccount,
                pool.pair.base.toString()
              );

              const series = calculatePredictedChartSeries(
                currentPrices[0][0],
                new BigNumber(baseBal),
                new BigNumber(quoteBal),
                pool,
                new BigNumber(blockNo.toString())
              );
              setPredictedPriceSeries(series);
            });
          }
        }
      });
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
