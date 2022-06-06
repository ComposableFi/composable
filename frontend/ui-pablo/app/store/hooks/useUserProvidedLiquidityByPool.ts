import { getAssetByOnChainId } from "@/defi/polkadot/Assets";
import { ConstantProductPool, StableSwapPool } from "@/store/pools/pools.types";
import useStore from "@/store/useStore";
import { calcaulateProvidedLiquidity } from "@/updaters/liquidity/utils";
import { liquidityTransactionsByAddressAndPool } from "@/updaters/pools/subsquid";
import BigNumber from "bignumber.js";
import { useEffect, useMemo, useState } from "react";
import { useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "../../updaters/constants";
import { useAllLpTokenRewardingPools } from "./useAllLpTokenRewardingPools";
/**
 * Provides the amount of liquidity
 * added by the user, and its value in
 * USD
 * @param poolId number
 * @returns {
 *  tokenAmounts: { baseAmount: BigNumber; quoteAmount: BigNumber };
 *  value: { baseValue: BigNumber; quoteValue: BigNumber };
 * }
 */
export const useUserProvidedLiquidityByPool = (
  poolId: number
): {
  tokenAmounts: { baseAmount: BigNumber; quoteAmount: BigNumber };
  value: { baseValue: BigNumber; quoteValue: BigNumber };
} => {
  /**
   * selected account on UI
   */
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const {
    /**
     * prices of assets within the
     * pool
     */
    assets,
    /**
     * acutal provided liquidity from zustand
     */
    userProvidedLiquidity,
    /**
     * set amounts, used in first effect
     */
    setUserProvidedTokenAmountInLiquidityPool,
  } = useStore();
  /**
   * All lp rewards pools
   * that allow swap, trading
   */
  const allPools = useAllLpTokenRewardingPools();
  /**
   * whichever pool is
   * selected
   */
  const pool = useMemo<StableSwapPool | ConstantProductPool | undefined>(() => {
    return allPools.find((i) => i.poolId === poolId);
  }, [undefined]);
  /**
   * hook defaults
   */
  const [liquidityProvided, setLiquidityProvided] = useState({
    tokenAmounts: {
      baseAmount: new BigNumber(0),
      quoteAmount: new BigNumber(0),
    },
  });
  const [value, setValue] = useState({
    baseValue: new BigNumber(0),
    quoteValue: new BigNumber(0),
  });
  /**
   * Fetch user provided liquidity
   * from subsquid
   */
  useEffect(() => {
    if (pool && selectedAccount) {
      liquidityTransactionsByAddressAndPool(
        selectedAccount.address,
        pool.poolId
      ).then((userLiqTransactions) => {
        console.log("Fetch up to data pool asset amounts");
        let { baseAmountProvided, quoteAmountProvided } =
          calcaulateProvidedLiquidity(
            userLiqTransactions.data.pabloTransactions
          );
        setUserProvidedTokenAmountInLiquidityPool((pool as any).poolId, {
          baseAmount: baseAmountProvided.toString(),
          quoteAmount: quoteAmountProvided.toString(),
        });
      });
    }
  }, [pool, selectedAccount]);
  /**
   * use amount of liquity tokens
   * from zustand store and pass it
   * down
   */
  useEffect(() => {
    if (pool) {
      if (userProvidedLiquidity[pool.poolId]) {
        setLiquidityProvided((p) => {
          return {
            ...p,
            tokenAmounts: {
              baseAmount: new BigNumber(
                userProvidedLiquidity[pool.poolId].tokenAmounts.baseAmount
              ),
              quoteAmount: new BigNumber(
                userProvidedLiquidity[pool.poolId].tokenAmounts.quoteAmount
              ),
            },
          };
        });
      }
    }
  }, [pool, userProvidedLiquidity]);
  /**
   * Update user base asset 
   * provided liquidity
   * value (in USD) in zustand store
   */
  useEffect(() => {
    if (pool) {
      const baseAssetMeta = getAssetByOnChainId(
        DEFAULT_NETWORK_ID,
        pool.pair.base
      );

      if (baseAssetMeta && assets[baseAssetMeta.assetId]) {
        setValue((v) => {
          return {
            ...v,
            baseValue: new BigNumber(
              liquidityProvided.tokenAmounts.baseAmount
            ).times(assets[baseAssetMeta.assetId].price),
          };
        });
      }
    }
  }, [pool, assets, liquidityProvided.tokenAmounts.baseAmount]);
  /**
   * Update user quote asset 
   * provided liquidity
   * value (in USD) in zustand store
   */
  useEffect(() => {
    if (pool) {
      const quoteAssetMeta = getAssetByOnChainId(
        DEFAULT_NETWORK_ID,
        pool.pair.quote
      );

      if (quoteAssetMeta && assets[quoteAssetMeta.assetId]) {
        setValue((v) => {
          return {
            ...v,
            quoteValue: new BigNumber(
              liquidityProvided.tokenAmounts.quoteAmount
            ).times(assets[quoteAssetMeta.assetId].price),
          };
        });
      }
    }
  }, [pool, assets, liquidityProvided.tokenAmounts.quoteAmount]);

  return { tokenAmounts: liquidityProvided.tokenAmounts, value };
};
