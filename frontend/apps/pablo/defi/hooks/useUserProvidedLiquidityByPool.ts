import { useEffect, useMemo, useState } from "react";
import { useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import { useAllLpTokenRewardingPools } from "./useAllLpTokenRewardingPools";
import { fetchLiquidityProvided } from "@/defi/subsquid/liquidity/helpers";
import { BasePabloPool } from "shared";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
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
} => {
  /**
   * selected account on UI
   */
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const {
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
  const pool = useMemo<BasePabloPool | undefined>(() => {
    return allPools.find((i) => {
      return (i.getPoolId(true) as BigNumber).toNumber() === poolId
    });
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
  /**
   * Fetch user provided liquidity
   * from subsquid
   */
  useEffect(() => {
    console.log("Query subsquid for user liquidity");
    if (pool && selectedAccount) {
      const poolId = (pool.getPoolId(true) as BigNumber).toNumber()
      fetchLiquidityProvided(
        selectedAccount.address,
        poolId
      ).then((liqRecord) => {
        setUserProvidedTokenAmountInLiquidityPool(
          poolId,
          liqRecord[poolId]
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
      const poolId = (pool.getPoolId(true) as BigNumber).toNumber()
      if (userProvidedLiquidity[poolId]) {
        setLiquidityProvided({
          tokenAmounts: {
            baseAmount: new BigNumber(
              userProvidedLiquidity[poolId].tokenAmounts.baseAmount
            ),
            quoteAmount: new BigNumber(
              userProvidedLiquidity[poolId].tokenAmounts.quoteAmount
            ),
          },
        });
      }
    }
  }, [pool, userProvidedLiquidity]);

  return { tokenAmounts: liquidityProvided.tokenAmounts };
};
