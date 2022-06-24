import { useEffect } from "react";
import { useParachainApi } from "substrate-react";
import { DEFAULT_NETWORK_ID, isValidAssetPair } from "@/defi/utils";
import { createPabloPoolAccountId } from "@/defi/utils";
import { fetchSpotPrice } from "@/defi/utils/pablo/spotPrice";
import {
  ConstantProductPool,
  StableSwapPool,
} from "@/defi/types";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { LiquidityPoolType } from "@/store/pools/pools.types";

const Updater = () => {
  const {
    swaps: {
      ui,
      poolConstants
    },
    setDexRouteSwaps,
    setPoolConstantsSwaps,
    setPoolVariablesSwaps,
    pools: {
      constantProductPools,
      stableSwapPools,
    },
    resetSwaps
  } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
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
              resetSwaps();
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
                  feeConfig: pool.feeConfig,
                  lbpConstants: lbp,
                  poolType,
                  pair,
                };

                setDexRouteSwaps([Number(poolId)]);
                setPoolConstantsSwaps(poolConstants);
              } else {
                resetSwaps();
              }
            } else {
              // Some future logic
            }
          }
        );
      }
    }
    /**
     * eslint asks 
     * 'setDexRouteSwaps', 'setPoolConstantsSwaps', and 'swaps'
     * to be added to dependancy list
     * 'setDexRouteSwaps', 'setPoolConstantsSwaps' are setters and theyll remain
     * the same throught the renders
     */
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [
    ui,
    parachainApi,
    constantProductPools.verified,
    stableSwapPools.verified,
  ]);

  useEffect(() => {
    if (poolConstants.poolIndex !== -1) {
      const { pair } = poolConstants;
      const { baseAssetSelected, quoteAssetSelected } = ui;
      if (
        isValidAssetPair(baseAssetSelected, quoteAssetSelected) &&
        parachainApi
      ) {
        const isReversedTrade = pair.base.toString() === baseAssetSelected;
        if (baseAssetSelected && quoteAssetSelected) {
          fetchSpotPrice(
            parachainApi,
            { base: poolConstants.pair.base.toString(), quote: poolConstants.pair.quote.toString() },
            poolConstants.poolIndex
          ).then(
            (spotPrice) => {
              if (isReversedTrade) {
                spotPrice = new BigNumber(1).div(spotPrice as BigNumber);
              }
              setPoolVariablesSwaps({
                spotPrice: (spotPrice as BigNumber).toFixed(4),
              });
            }
          );
        }
      }
    } else {
      setPoolVariablesSwaps({
        spotPrice: "0",
      });
    }
  }, [ui, poolConstants, parachainApi, setPoolVariablesSwaps]);

  return null;
};

export default Updater;
