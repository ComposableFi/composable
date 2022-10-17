import { useEffect, useMemo, useState } from "react";
import { useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import { useAllLpTokenRewardingPools } from "./useAllLpTokenRewardingPools";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { ConstantProductPool, StableSwapPool } from "@/defi/types";
import { fetchLiquidityProvided } from "@/defi/subsquid/liquidity/helpers";
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
    apollo,
    /**
     * actual provided liquidity from zustand
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
  }, [poolId, allPools]);
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
    console.log("Query subsquid for user liquidity");
    if (pool && selectedAccount) {
      fetchLiquidityProvided(
        selectedAccount.address,
        pool.poolId
      ).then((liqRecord) => {
        setUserProvidedTokenAmountInLiquidityPool(
          pool.poolId,
          liqRecord[pool.poolId]
        );
      });
    }
  }, [pool, selectedAccount, setUserProvidedTokenAmountInLiquidityPool]);
  /**
   * use amount of liquidity tokens
   * from zustand store and pass it
   * down
   */
  useEffect(() => {
    if (pool) {
      if (userProvidedLiquidity[pool.poolId]) {
        setLiquidityProvided({
          tokenAmounts: {
            baseAmount: new BigNumber(
              userProvidedLiquidity[pool.poolId].tokenAmounts.baseAmount
            ),
            quoteAmount: new BigNumber(
              userProvidedLiquidity[pool.poolId].tokenAmounts.quoteAmount
            ),
          },
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
    if (pool && apollo[pool.pair.base.toString()]) {
      setValue((v) => {
        return {
          ...v,
          baseValue: new BigNumber(
            liquidityProvided.tokenAmounts.baseAmount
          ).times(apollo[pool.pair.base.toString()]),
        };
      });
    }
  }, [pool, apollo, liquidityProvided.tokenAmounts.baseAmount]);
  /**
   * Update user quote asset
   * provided liquidity
   * value (in USD) in zustand store
   */
  useEffect(() => {
    if (pool && apollo[pool.pair.quote.toString()]) {
      setValue((v) => {
        return {
          ...v,
          quoteValue: new BigNumber(
            liquidityProvided.tokenAmounts.quoteAmount
          ).times(apollo[pool.pair.quote.toString()]),
        };
      });
    }
  }, [pool, apollo, liquidityProvided.tokenAmounts.quoteAmount]);

  return { tokenAmounts: liquidityProvided.tokenAmounts, value };
};
