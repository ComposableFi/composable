import { useEffect } from "react";
import useStore from "@/store/useStore";
import { useParachainApi } from "substrate-react";
import { AssetId } from "@/defi/polkadot/types";
import { Assets, getAssetOnChainId } from "@/defi/polkadot/Assets";
import BigNumber from "bignumber.js";

import { subsquidClient } from "@/subsquid";
import {
  swapTransactionsToChartSeries,
  fetchSpotPrice,
  transformSwapSubsquidTx,
} from "./utils";
import { createPoolAccountId, isValidAssetPair } from "../utils";
import {
  ConstantProductPool,
  LiquidityBootstrappingPool,
  LiquidityPoolType,
  StableSwapPool,
} from "@/store/pools/pools.types";
import { fetchBalanceByAssetId } from "../balances/utils";
import { query24hOldTransactionByPoolQuoteAsset } from "./subsquid";
import { queryPoolTransactionsByType } from "../pools/subsquid";

const Updater = () => {
  const {
    assets,
    swaps,
    swapsChart,
    setDexRouteSwaps,
    setPoolConstantsSwaps,
    setUserAccountBalanceSwaps,
    setPoolVariablesSwaps,
    putSwapsChartSeries,
    put24HourOldPrice,
    pools: {
      liquidityBootstrappingPools,
      constantProductPools,
      stableSwapPools,
    },
  } = useStore();
  const { parachainApi } = useParachainApi("picasso");
  /**
   * Triggered when user changes first
   * token from token list dropdown on
   * swaps page
   * Updates with balance from zustand store
   */
  useEffect(() => {
    const { ui } = swaps;
    if (ui.quoteAssetSelected === "none") {
      setUserAccountBalanceSwaps("quote", "0");
    } else {
      const balance = assets[ui.quoteAssetSelected as AssetId].balance.picasso;
      setUserAccountBalanceSwaps("quote", balance);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [swaps.ui, assets]);
  /**
   * Triggered when user changes second
   * token from token list dropdown on
   * swaps page
   * Updates with balance from zustand store
   */
  useEffect(() => {
    const { ui } = swaps;
    if (ui.baseAssetSelected === "none") {
      setUserAccountBalanceSwaps("base", "0");
    } else {
      const balance = assets[ui.baseAssetSelected as AssetId].balance.picasso;
      setUserAccountBalanceSwaps("base", balance);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [swaps.ui, assets]);
  /**
   * This hook is triggered when all
   * pools are fetched from the pablo pallet
   *
   * It is responsible for two things
   * 1. Fetch and check permissioned route for
   *    selected asset pair in UI
   * 2. Update swaps zustand store with pool constants
   *    fetched from the chain
   */
  useEffect(() => {
    const { ui } = swaps;
    if (isValidAssetPair(ui.baseAssetSelected, ui.quoteAssetSelected)) {
      if (
        parachainApi &&
        (liquidityBootstrappingPools.verified.length ||
          constantProductPools.verified.length ||
          stableSwapPools.verified.length)
      ) {
        // Convert to on chain Asset Ids from string Asset Ids
        const _baseAssetId =
          Assets[ui.baseAssetSelected as AssetId].supportedNetwork.picasso;
        const _quoteAssetId =
          Assets[ui.quoteAssetSelected as AssetId].supportedNetwork.picasso;
        /**
         * Fetch a permissioned route, dexRoutes do
         * not support inverted routes so we query
         * both possibilites
         */
        const routePromises = [
          parachainApi.query.dexRouter.dexRoutes(_baseAssetId, _quoteAssetId),
          parachainApi.query.dexRouter.dexRoutes(_quoteAssetId, _baseAssetId),
        ];

        Promise.all(routePromises).then(
          ([baseToQuoteRoute, quoteToBaseRoute]) => {
            let baseToQuoteRouteJSON = baseToQuoteRoute.toJSON();
            let quoteToBaseRouteJSON = quoteToBaseRoute.toJSON();

            let dexRoute: any = null;
            if (!!baseToQuoteRouteJSON) dexRoute = baseToQuoteRouteJSON;
            if (!!quoteToBaseRouteJSON) dexRoute = quoteToBaseRouteJSON;

            if (dexRoute === null) {
              /**
               * Clear Data here as no
               * permissioned route was found
               */
              setDexRouteSwaps([]);
              setPoolConstantsSwaps({
                poolAccountId: "",
                poolIndex: -1,
                lbpConstants: undefined,
                poolType: "none",
                fee: "0",
                pair: { quote: -1, base: -1 },
              });
            } else if (dexRoute.direct) {
              /**
               * found a route, involves a single
               * pool which is why "direct"
               */
              let poolType: LiquidityPoolType | "none" = "none";
              let pair = { quote: -1, base: -1 };
              let poolId = dexRoute.direct[0];
              poolId = poolId.toString();

              /**
               * For swapping tokens, we only allow UI
               * to use dexRouter registered pools
               */
              let pool:
                | LiquidityBootstrappingPool
                | ConstantProductPool
                | StableSwapPool
                | undefined = liquidityBootstrappingPools.verified.find(
                (pool) => pool.poolId.toString() === poolId
              );
              if (!pool) {
                poolType = "ConstantProduct";
                pool = constantProductPools.verified.find(
                  (pool) => pool.poolId.toString() === poolId
                );
              }
              if (pool && !(pool as any).sale) {
                poolType = "ConstantProduct";
                pool = stableSwapPools.verified.find(
                  (pool) => pool.poolId.toString() === poolId
                );
              }

              if (pool) {
                let poolAccountId = createPoolAccountId(poolId as number);

                let lbp = undefined;
                let fee = new BigNumber(pool.fee);
                pair = pool.pair;

                if ((pool as LiquidityBootstrappingPool).sale) {
                  poolType = "LiquidityBootstrapping";
                  lbp = {
                    start: (pool as LiquidityBootstrappingPool).sale.start,
                    end: (pool as LiquidityBootstrappingPool).sale.end,
                    initialWeight: (
                      pool as LiquidityBootstrappingPool
                    ).sale.initialWeight.toString(),
                    finalWeight: (
                      pool as LiquidityBootstrappingPool
                    ).sale.finalWeight.toString(),
                  };
                }

                if ((pool as ConstantProductPool).ownerFee) {
                  fee.plus((pool as ConstantProductPool).ownerFee);
                }

                let poolConstants = {
                  poolAccountId: poolAccountId,
                  poolIndex: Number(poolId),
                  fee: fee.toString(),
                  lbpConstants: lbp,
                  poolType,
                  pair,
                };

                setDexRouteSwaps([Number(poolId)]);
                setPoolConstantsSwaps(poolConstants);
              }
            } else {
              // Some future logic
            }
          }
        );
      }
    }
  }, [
    swaps.ui,
    parachainApi,
    liquidityBootstrappingPools.verified.length,
    constantProductPools.verified.length,
    stableSwapPools.verified.length,
  ]);

  useEffect(() => {
    if (swaps.poolConstants.poolIndex !== -1) {
      const { poolAccountId, pair } = swaps.poolConstants;
      const { baseAssetSelected, quoteAssetSelected } = swaps.ui;

      if (
        isValidAssetPair(baseAssetSelected, quoteAssetSelected) &&
        parachainApi
      ) {
        const _baseAssetId =
          Assets[baseAssetSelected as AssetId].supportedNetwork.picasso;
        const _quoteAssetId =
          Assets[quoteAssetSelected as AssetId].supportedNetwork.picasso;

        const isReversedTrade = pair.base === _baseAssetId;

        if (_baseAssetId && _quoteAssetId) {
          let promises = [
            fetchBalanceByAssetId(
              parachainApi,
              "picasso",
              poolAccountId,
              _baseAssetId.toString()
            ),
            fetchBalanceByAssetId(
              parachainApi,
              "picasso",
              poolAccountId,
              _quoteAssetId.toString()
            ),
            fetchSpotPrice(
              parachainApi,
              swaps.poolConstants.pair,
              swaps.poolConstants.poolIndex
            ),
            subsquidClient
              .query(
                query24hOldTransactionByPoolQuoteAsset(
                  swaps.poolConstants.poolIndex,
                  _quoteAssetId,
                  1
                )
              )
              .toPromise(),
          ];

          Promise.all(promises).then(
            ([
              baseAssetBalance,
              quoteAssetBalance,
              spotPrice,
              priceChangeResponse,
            ]) => {
              if (isReversedTrade) {
                spotPrice = new BigNumber(1).div(spotPrice as BigNumber);
              }

              setPoolVariablesSwaps({
                spotPrice: (spotPrice as BigNumber).toFixed(4),
                baseAssetReserve: baseAssetBalance as string,
                quoteAssetReserve: quoteAssetBalance as string,
              });

              if (
                (priceChangeResponse as any).data &&
                (priceChangeResponse as any).data.pabloTransactions
              ) {
                let pc = new BigNumber(0);
                if ((priceChangeResponse as any).data.pabloTransactions[0]) {
                  pc = new BigNumber(
                    (
                      priceChangeResponse as any
                    ).data.pabloTransactions[0].spotPrice
                  );
                }
                put24HourOldPrice(pc.toString());
              }
            }
          );
        }
      }
    } else {
      setPoolVariablesSwaps({
        spotPrice: "0",
        baseAssetReserve: "0",
        quoteAssetReserve: "0",
      });
    }
  }, [swaps.ui, swaps.poolConstants.poolIndex, parachainApi]);

  useEffect(() => {
    const { ui, poolConstants } = swaps;
    if (
      parachainApi &&
      poolConstants.poolIndex !== -1 &&
      isValidAssetPair(ui.baseAssetSelected, ui.quoteAssetSelected)
    ) {
      const _selectedQuoteAssetId = getAssetOnChainId(
        "picasso",
        ui.quoteAssetSelected as AssetId
      );

      if (_selectedQuoteAssetId) {
        subsquidClient
          .query(
            queryPoolTransactionsByType(poolConstants.poolIndex, "SWAP", 250)
          )
          .toPromise()
          .then((response) => {
            if (
              response.data &&
              response.data.pabloTransactions &&
              response.data.pabloTransactions.length
            ) {
              const txs = transformSwapSubsquidTx(
                response.data.pabloTransactions,
                _selectedQuoteAssetId
              );

              putSwapsChartSeries(
                swapTransactionsToChartSeries(txs, swapsChart.selectedRange)
              );
            } else {
              putSwapsChartSeries([]);
            }
          });
      } else {
        putSwapsChartSeries([]);
      }
    } else {
      putSwapsChartSeries([]);
    }
  }, [
    swaps.ui,
    swaps.poolConstants.poolIndex,
    parachainApi,
    swapsChart.selectedRange,
  ]);

  return null;
};

export default Updater;
