import { DEFI_CONFIG } from "@/defi/config";
import { queryPoolTransactionsByType } from "@/updaters/pools/subsquid";
import { query24hOldTransactionByPoolQuoteAsset } from "@/updaters/swaps/subsquid";
import { useState, useEffect } from "react";
import BigNumber from "bignumber.js";

import useStore from "../useStore";
import {
  ChartRange,
  processSubsquidChartData,
} from "@/defi/utils/charts";

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
      queryPoolTransactionsByType(selectedPool.poolId, "SWAP", 250).then((response) => {
        if (
          response.data?.pabloTransactions?.length
        ) {
          let swapTransactions = response.data.pabloTransactions.map(
            (tx: {
              baseAssetId: string;
              quoteAssetId: string;
              receivedTimestamp: string;
              spotPrice: string;
            }) => {
              let spotPrice = new BigNumber(tx.spotPrice);
              if (tx.quoteAssetId !== quote) {
                spotPrice = new BigNumber(1).div(tx.spotPrice);
              }

              return [Number(tx.receivedTimestamp), spotPrice];
            }
          );

          setChartSeries(
            processSubsquidChartData(
              swapTransactions,
              selectedInterval.symbol as ChartRange
            )
          );
        } else {
          setChartSeries([]);
        }
      });
    }
  }, [selectedPool, quote, selectedInterval]);

  useEffect(() => {
    if (selectedPool && selectedPool.poolId !== -1 && quote !== "none") {
      query24hOldTransactionByPoolQuoteAsset(
        selectedPool.poolId,
        +quote,
        "SWAP",
        1
      ).then((response) => {
        if (
          (response as any).data &&
          (response as any).data.pabloTransactions
        ) {
          let pc = new BigNumber(0);
          if ((response as any).data.pabloTransactions[0]) {
            pc = new BigNumber(
              (response as any).data.pabloTransactions[0].spotPrice
            );
          }
          set24HourOldPrice(pc);
        }
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
