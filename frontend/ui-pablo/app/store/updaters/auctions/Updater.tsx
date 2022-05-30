import { useEffect } from "react";
import { useParachainApi } from "substrate-react";
import useStore from "@/store/useStore";
import {
  subsquidClient,
} from "@/subsquid";
import { getAssetById } from "@/defi/polkadot/Assets";
import BigNumber from "bignumber.js";

import { aggregateTrades, transformAuctionsTransaction } from "./utils";
import { PoolTradeHistory } from "@/store/auctions/auctions.types";
import { createPoolAccountId } from "../utils";
import { fetchBalanceByAssetId } from "../balances/utils";
import { AVERAGE_BLOCK_TIME, DEFAULT_DECIMALS } from "../pools/constants";
import { queryAuctionStats } from "./subsquid";
import { queryPoolTransactionsByType } from "../pools/subsquid";
import { fetchSpotPrice } from "../swaps/utils";

const Updater = () => {
  const {
    pools: {
      liquidityBootstrappingPools,
      setLiquidityBootstrappingPoolSpotPrice,
    },
    auctions,
    putStatsActiveLBP,
    putHistoryActiveLBP,
    putChartSeries,
  } = useStore();
  const { parachainApi } = useParachainApi("picasso");
  /**
   * Queries initiated on an Auctions
   * LBP selection
   */
  useEffect(() => {
    if (parachainApi && auctions.activeLBP.poolId !== -1) {
      const { base, quote } = auctions.activeLBP.pair;
      const { start } = auctions.activeLBP.sale;

      const baseAsset = getAssetById("picasso", base);
      const quoteAsset = getAssetById("picasso", quote);
      const baseDecimals = baseAsset
        ? new BigNumber(10).pow(baseAsset.decimals)
        : DEFAULT_DECIMALS;
      const quoteDecimals = quoteAsset
        ? new BigNumber(10).pow(quoteAsset.decimals)
        : DEFAULT_DECIMALS;

      const poolAccountId = createPoolAccountId(auctions.activeLBP.poolId);

      let allQueries = [
        /**
         * Query for volume, liquidity
         */
        subsquidClient
          .query(queryAuctionStats(auctions.activeLBP.poolId))
          .toPromise(),
        /**
         * Query trade history
         * for transactions tab
         */
        subsquidClient
          .query(
            queryPoolTransactionsByType(auctions.activeLBP.poolId, "SWAP")
          )
          .toPromise(),
        /**
         * Query for initial balances
         */
        subsquidClient
          .query(
            queryPoolTransactionsByType(
              auctions.activeLBP.poolId,
              "ADD_LIQUIDITY"
            )
          )
          .toPromise(),
        /**
         * Query for calculating initial balances
         * before sale starts
         */
        subsquidClient
          .query(
            queryPoolTransactionsByType(
              auctions.activeLBP.poolId,
              "CREATE_POOL"
            )
          )
          .toPromise(),
        /**
         * Query amount of base tokens in
         * the pool
         */
        fetchBalanceByAssetId(
          parachainApi,
          "picasso",
          poolAccountId,
          base.toString()
        ),
        /**
         * Query amount of quote tokens in
         * the pool
         */
        fetchBalanceByAssetId(
          parachainApi,
          "picasso",
          poolAccountId,
          quote.toString()
        ),
      ];

      Promise.all(allQueries).then(
        ([
          poolStats,
          swapsHistory,
          addLiq,
          createPool,
          baseBalance,
          quoteBalance,
        ]) => {
          let totalLiquidity = "0",
            totalVolume = "0";

          if (
            (poolStats as any).data &&
            (poolStats as any).data.pabloPools.length
          ) {
            totalLiquidity = (poolStats as any).data.pabloPools[0]
              .totalLiquidity;
            totalVolume = (poolStats as any).data.pabloPools[0].totalVolume;
          }

          let createPoolTx: PoolTradeHistory | undefined;
          if (
            (createPool as any).data &&
            (createPool as any).data.pabloTransactions
          ) {
            createPoolTx = transformAuctionsTransaction(
              (createPool as any).data.pabloTransactions[0],
              {
                baseDecimals,
                quoteDecimals,
                onChainPoolQuoteAssetId: quote,
              }
            );
          }

          let initialBalanceQuote = new BigNumber(0);
          let initialBalanceBase = new BigNumber(0);
          if ((addLiq as any).data && (addLiq as any).data.pabloTransactions) {
            const addLiqTxs: PoolTradeHistory[] = (
              addLiq as any
            ).data.pabloTransactions.map((t: any) =>
              transformAuctionsTransaction(t, {
                baseDecimals,
                quoteDecimals,
                onChainPoolQuoteAssetId: quote,
              })
            );

            let saleStartTs = createPoolTx
              ? createPoolTx.receivedTimestamp + AVERAGE_BLOCK_TIME * start
              : 0;

            initialBalanceQuote = addLiqTxs.reduce((agg, i) => {
              return i.receivedTimestamp < saleStartTs
                ? agg.plus(i.quoteAssetAmount)
                : agg;
            }, new BigNumber(0));
            initialBalanceBase = addLiqTxs.reduce((agg, i) => {
              return i.receivedTimestamp < saleStartTs
                ? agg.plus(i.baseAssetAmount)
                : agg;
            }, new BigNumber(0));

            putStatsActiveLBP({
              startBalances: {
                quote: initialBalanceQuote.toString(),
                base: initialBalanceBase.toString(),
              },
              currentBalances: {
                quote: quoteBalance as string,
                base: baseBalance as string,
              },
              liquidity: new BigNumber(totalLiquidity)
                .div(DEFAULT_DECIMALS)
                .toFixed(4),
              totalVolume: new BigNumber(totalVolume)
                .div(DEFAULT_DECIMALS)
                .toFixed(4),
            });
          }

          let swapTxs: PoolTradeHistory[] = [];
          if (
            (swapsHistory as any).data &&
            (swapsHistory as any).data.pabloTransactions
          ) {
            swapTxs = (swapsHistory as any).data.pabloTransactions.map(
              (t: any) =>
                transformAuctionsTransaction(t, {
                  baseDecimals,
                  quoteDecimals,
                  onChainPoolQuoteAssetId: quote,
                })
            );
            putChartSeries("price", aggregateTrades(swapTxs));
            putHistoryActiveLBP(swapTxs.slice(0, 10));
          }
        }
      );
    } else {
      putChartSeries("price", []);
      putHistoryActiveLBP([]);
    }
  }, [parachainApi, auctions.activeLBP]);

  /**
   * This effect is called to show prices
   * on auctions page
   */
  useEffect(() => {
    if (parachainApi && liquidityBootstrappingPools.verified.length > 0) {
      for (
        let pool = 0;
        pool < liquidityBootstrappingPools.verified.length;
        pool++
      ) {
        fetchSpotPrice(
          parachainApi,
          liquidityBootstrappingPools.verified[pool].pair,
          liquidityBootstrappingPools.verified[pool].poolId
        ).then((spotPrice) => {
          setLiquidityBootstrappingPoolSpotPrice(
            liquidityBootstrappingPools.verified[pool].poolId,
            spotPrice.toFixed(4)
          );
        });
      }
    }
  }, [parachainApi, liquidityBootstrappingPools.verified.length]);

  return null;
};

export default Updater;
