import { DEFI_CONFIG } from "@/defi/config";
import { getAssetOnChainId } from "@/defi/polkadot/Assets";
import { DEFAULT_NETWORK_ID } from "@/updaters/constants";
import { queryPoolTransactionsByType } from "@/updaters/pools/subsquid";
import { query24hOldTransactionByPoolQuoteAsset } from "@/updaters/swaps/subsquid";
import { useState, useEffect } from "react";
import BigNumber from "bignumber.js";

import useStore from "../useStore";
import {
  ChartRange,
  processSubsquidChartData,
} from "@/utils/defi/charts";

export const useSwapsChart = () => {
  const { swaps } = useStore();
  const { quoteAssetSelected } = swaps.ui;
  const { poolIndex } = swaps.poolConstants;

  const [seriesIntervals, setSeriesIntervals] = useState<string[]>([]);
  const [chartSeries, setChartSeries] = useState<[number, number][]>([]);
  const [selectedInterval, setSelectedInterval] = useState(
    DEFI_CONFIG.swapChartIntervals[0]
  );
  const [_24hourOldPrice, set24HourOldPrice] = useState(new BigNumber(0));

  useEffect(() => {
    if (poolIndex !== -1 && quoteAssetSelected !== "none") {
      const quoteAssetId = getAssetOnChainId(
        DEFAULT_NETWORK_ID,
        quoteAssetSelected
      );

      if (quoteAssetId) {
        queryPoolTransactionsByType(poolIndex, "SWAP", 250).then((response) => {
          if (
            response.data &&
            response.data.pabloTransactions &&
            response.data.pabloTransactions.length
          ) {
            let swapTransactions = response.data.pabloTransactions.map(
              (tx: {
                baseAssetId: string;
                quoteAssetId: string;
                receivedTimestamp: string;
                spotPrice: string;
              }) => {
                let spotPrice = new BigNumber(tx.spotPrice);
                if (tx.quoteAssetId !== quoteAssetId.toString()) {
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
    }
  }, [poolIndex, quoteAssetSelected, selectedInterval]);

  useEffect(() => {
    if (poolIndex !== -1 && quoteAssetSelected !== "none") {
      const quoteAssetId = getAssetOnChainId(
        DEFAULT_NETWORK_ID,
        quoteAssetSelected
      );
      if (quoteAssetId) {
        query24hOldTransactionByPoolQuoteAsset(
          swaps.poolConstants.poolIndex,
          quoteAssetId,
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
    }
  }, [poolIndex, quoteAssetSelected]);

  return {
    selectedInterval,
    setSelectedInterval,
    chartSeries,
    seriesIntervals,
    _24hourOldPrice,
  };
};
