import { useEffect } from "react";
import { useParachainApi } from "substrate-react";
import useStore from "@/store/useStore";
import { getAssetById } from "@/defi/polkadot/Assets";
import BigNumber from "bignumber.js";

import { aggregateTrades } from "./utils";
import { PoolTradeHistory } from "@/store/auctions/auctions.types";
import { fetchBalanceByAssetId, fromChainUnits } from "@/defi/utils";
import {
  AVERAGE_BLOCK_TIME,
  DEFAULT_DECIMALS,
  DEFAULT_NETWORK_ID,
} from "@/defi/utils/constants";
import { queryAuctionStats } from "./subsquid";
import { queryPoolTransactionsByType } from "../pools/subsquid";
import { fetchSpotPrice } from "@/defi/utils";
import { createPabloPoolAccountId } from "@/defi/utils/pablo";
import { transformAuctionsTransaction } from "@/defi/utils/pablo/auctions";

const Updater = () => {
  const {
    apollo,
    pools: {
      liquidityBootstrappingPools,
      setLiquidityBootstrappingPoolSpotPrice,
    },
    auctions,
    putStatsActiveLBP,
    putHistoryActiveLBP,
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

      const poolAccountId = createPabloPoolAccountId(
        parachainApi,
        auctions.activeLBP.poolId
      );

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
        fetchBalanceByAssetId(parachainApi, poolAccountId, base.toString()),
        /**
         * Query amount of quote tokens in
         * the pool
         */
        fetchBalanceByAssetId(parachainApi, poolAccountId, quote.toString()),
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
              quote
            );
          }

          let initialBalanceQuote = new BigNumber(0);
          let initialBalanceBase = new BigNumber(0);
          if ((addLiq as any).data && (addLiq as any).data.pabloTransactions) {
            const addLiqTxs: PoolTradeHistory[] = (
              addLiq as any
            ).data.pabloTransactions.map((t: any) =>
              transformAuctionsTransaction(t, quote)
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
              liquidity: fromChainUnits(totalLiquidity).toString(),
              totalVolume: fromChainUnits(totalVolume).toString()
            });
          }

          let swapTxs: PoolTradeHistory[] = [];
          if (
            (swapsHistory as any).data &&
            (swapsHistory as any).data.pabloTransactions
          ) {
            swapTxs = (swapsHistory as any).data.pabloTransactions.map(
              (t: any) => transformAuctionsTransaction(t, quote)
            );
            putHistoryActiveLBP(swapTxs.slice(0, 10));
          }
        }
      );
    } else {
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
        let quoteId =
          liquidityBootstrappingPools.verified[pool].pair.quote.toString();
        if (apollo[quoteId]) {
          fetchSpotPrice(
            parachainApi,
            liquidityBootstrappingPools.verified[pool].pair,
            liquidityBootstrappingPools.verified[pool].poolId
          ).then((spotPrice) => {
            spotPrice = spotPrice.times(apollo[quoteId]);
            setLiquidityBootstrappingPoolSpotPrice(
              liquidityBootstrappingPools.verified[pool].poolId,
              spotPrice.toFixed(4)
            );
          });
        }
      }
    }
  }, [parachainApi, liquidityBootstrappingPools.verified.length, apollo]);

  return null;
};

export default Updater;
