import { useEffect } from "react";
import { useParachainApi } from "substrate-react";
import { DEFAULT_NETWORK_ID, isValidAssetPair } from "@/defi/utils";
import { fetchBalanceByAssetId } from "@/defi/utils";
import { createPabloPoolAccountId } from "@/defi/utils";
import { fetchSpotPrice } from "@/defi/utils/pablo/spotPrice";
import {
  ConstantProductPool,
  LiquidityPoolType,
  StableSwapPool,
} from "@/store/pools/pools.types";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";

const Updater = () => {
  const {
    assetBalances,
    swaps,
    setDexRouteSwaps,
    setPoolConstantsSwaps,
    setUserAccountBalanceSwaps,
    setPoolVariablesSwaps,
    pools: {
      constantProductPools,
      stableSwapPools,
    },
  } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  /**
   * Triggered when user changes first
   * token from token list dropdown on
   * swaps page
   * Updates with balance from zustand store
   */
  useEffect(() => {
    const { ui } = swaps;
    if (assetBalances[DEFAULT_NETWORK_ID]) {
      if (assetBalances[DEFAULT_NETWORK_ID][ui.baseAssetSelected]) {
        const balance = assetBalances[DEFAULT_NETWORK_ID][ui.baseAssetSelected];
        setUserAccountBalanceSwaps("base", balance);
      }
    }
    setUserAccountBalanceSwaps("base", "0");
  }, [swaps, assetBalances, setUserAccountBalanceSwaps]);
  /**
   * Triggered when user changes second
   * token from token list dropdown on
   * swaps page
   * Updates with balance from zustand store
   */
  useEffect(() => {
    const { ui } = swaps;
    if (assetBalances[DEFAULT_NETWORK_ID]) {
      if (assetBalances[DEFAULT_NETWORK_ID][ui.quoteAssetSelected]) {
        const balance = assetBalances[DEFAULT_NETWORK_ID][ui.quoteAssetSelected];
        setUserAccountBalanceSwaps("quote", balance);
      }
    }
    setUserAccountBalanceSwaps("quote", "0");
  }, [swaps, assetBalances, setUserAccountBalanceSwaps]);
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
        // (liquidityBootstrappingPools.verified.length ||
          (constantProductPools.verified.length ||
          stableSwapPools.verified.length)
      ) {
        /**
         * Fetch a permissioned route, dexRoutes do
         * not support inverted routes so we query
         * both possibilites
         */
        const routePromises = [
          parachainApi.query.dexRouter.dexRoutes(ui.baseAssetSelected, ui.quoteAssetSelected),
          parachainApi.query.dexRouter.dexRoutes(ui.quoteAssetSelected, ui.baseAssetSelected),
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
                | ConstantProductPool
                | StableSwapPool
                | undefined;
                
              //   = liquidityBootstrappingPools.verified.find(
              //   (pool) => pool.poolId.toString() === poolId
              // );
              if (!pool) {
                poolType = "ConstantProduct";
                pool = constantProductPools.verified.find(
                  (pool) => pool.poolId.toString() === poolId
                );
              }
              if (!pool) {
                poolType = "StableSwap";
                pool = stableSwapPools.verified.find(
                  (pool) => pool.poolId.toString() === poolId
                );
              }

              if (pool) {
                let poolAccountId = createPabloPoolAccountId(
                  parachainApi,
                  Number(poolId)
                );

                let lbp = undefined;
                let fee = new BigNumber(pool.feeConfig.feeRate);
                pair = pool.pair;

                // if ((pool as LiquidityBootstrappingPool).sale) {
                //   poolType = "LiquidityBootstrapping";
                //   lbp = {
                //     start: (pool as LiquidityBootstrappingPool).sale.start,
                //     end: (pool as LiquidityBootstrappingPool).sale.end,
                //     initialWeight: (
                //       pool as LiquidityBootstrappingPool
                //     ).sale.initialWeight.toString(),
                //     finalWeight: (
                //       pool as LiquidityBootstrappingPool
                //     ).sale.finalWeight.toString(),
                //   };
                // }

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
              } else {
                setDexRouteSwaps([]);
                setPoolConstantsSwaps({
                  poolAccountId: "",
                  poolIndex: -1,
                  lbpConstants: undefined,
                  poolType: "none",
                  fee: "0",
                  pair: { quote: -1, base: -1 },
                });
              }
            } else {
              // Some future logic
            }
          }
        );
      }
    }
  }, [
    swaps,
    parachainApi,
    constantProductPools.verified,
    stableSwapPools.verified,
    setDexRouteSwaps,
    setPoolConstantsSwaps,
  ]);

  useEffect(() => {
    if (swaps.poolConstants.poolIndex !== -1) {
      const { poolAccountId, pair } = swaps.poolConstants;
      const { baseAssetSelected, quoteAssetSelected } = swaps.ui;

      if (
        isValidAssetPair(baseAssetSelected, quoteAssetSelected) &&
        parachainApi
      ) {

        const isReversedTrade = pair.base.toString() === baseAssetSelected;

        if (baseAssetSelected && quoteAssetSelected) {
          let promises = [
            fetchBalanceByAssetId(
              parachainApi,
              poolAccountId,
              baseAssetSelected
            ),
            fetchBalanceByAssetId(
              parachainApi,
              poolAccountId,
              quoteAssetSelected
            ),
            fetchSpotPrice(
              parachainApi,
              { base: swaps.poolConstants.pair.base.toString(), quote: swaps.poolConstants.pair.quote.toString() },
              swaps.poolConstants.poolIndex
            ),
          ];

          Promise.all(promises).then(
            ([baseAssetBalance, quoteAssetBalance, spotPrice]) => {
              if (isReversedTrade) {
                spotPrice = new BigNumber(1).div(spotPrice as BigNumber);
              }

              setPoolVariablesSwaps({
                spotPrice: (spotPrice as BigNumber).toFixed(4),
                baseAssetReserve: baseAssetBalance as string,
                quoteAssetReserve: quoteAssetBalance as string,
              });
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
  }, [swaps.ui, swaps.poolConstants, parachainApi, setPoolVariablesSwaps]);

  return null;
};

export default Updater;
