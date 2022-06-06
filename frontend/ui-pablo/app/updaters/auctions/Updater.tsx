import { useEffect } from "react";
import { useParachainApi } from "substrate-react";
import useStore from "@/store/useStore";
import { getAssetById } from "@/defi/polkadot/Assets";
import BigNumber from "bignumber.js";

import { aggregateTrades, transformAuctionsTransaction } from "./utils";
import { PoolTradeHistory } from "@/store/auctions/auctions.types";
import { fetchBalanceByAssetId } from "../balances/utils";
import { AVERAGE_BLOCK_TIME, DEFAULT_DECIMALS, DEFAULT_NETWORK_ID } from "../constants";
import { queryAuctionStats } from "./subsquid";
import { queryPoolTransactionsByType } from "../pools/subsquid";
import { fetchSpotPrice } from "../swaps/utils";
import { createPoolAccountId } from "@/utils/substrate";

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
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  /**
   * Queries initiated on an Auctions
   * LBP selection
   */
  useEffect(() => {
    const { poolId } = auctions.activeLBP;
    if (parachainApi && poolId !== -1) {
      const { base, quote } = auctions.activeLBP.pair;
      const { start } = auctions.activeLBP.sale;

      const baseAsset = getAssetById(DEFAULT_NETWORK_ID, base);
      const quoteAsset = getAssetById(DEFAULT_NETWORK_ID, quote);
      const baseDecimals = baseAsset
        ? new BigNumber(10).pow(baseAsset.decimals)
        : DEFAULT_DECIMALS;
      const quoteDecimals = quoteAsset
        ? new BigNumber(10).pow(quoteAsset.decimals)
        : DEFAULT_DECIMALS;

      const poolAccountId = createPoolAccountId(parachainApi, auctions.activeLBP.poolId);

      let allQueries = [
        /**
         * Query for volume, liquidity
         */
        queryAuctionStats(poolId),
        /**
         * Query trade history
         * for transactions tab
         */
        queryPoolTransactionsByType(auctions.activeLBP.poolId, "SWAP"),
        /**
         * Query for initial balances
         */
        queryPoolTransactionsByType(auctions.activeLBP.poolId, "ADD_LIQUIDITY"),
        /**
         * Query for calculating initial balances
         * before sale starts
         */
        queryPoolTransactionsByType(auctions.activeLBP.poolId, "CREATE_POOL"),
        /**
         * Query amount of base tokens in
         * the pool
         */
        fetchBalanceByAssetId(
          parachainApi,
          DEFAULT_NETWORK_ID,
          poolAccountId,
          base.toString()
        ),
        /**
         * Query amount of quote tokens in
         * the pool
         */
        fetchBalanceByAssetId(
          parachainApi,
          DEFAULT_NETWORK_ID,
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

          /**
           * Show liquidity in Base Asset
           * can be changed in future
           */
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
                .div(quoteDecimals)
                .toFixed(4),
              totalVolume: new BigNumber(totalVolume)
                .div(quoteDecimals)
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
