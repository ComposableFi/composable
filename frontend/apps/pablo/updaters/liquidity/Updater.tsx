import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { useEffect, useMemo } from "react";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import { fetchAndUpdatePoolLiquidity } from "@/defi/utils";
import { fetchBalanceByAssetId } from "@/defi/utils";
import _ from "lodash";

const PICK = ["poolId", "pair", "lpToken"];
const Updater = () => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const {
    apollo,
    pools,
    setTokenAmountInLiquidityPool,
    setTokenValueInLiquidityPool,
    setUserLpBalance,
    poolLiquidity,
  } = useStore();
  /**
   * Select pools tracking
   * liquidity
   */
  const allPools = useMemo(() => {
    return [
      // ...pools.constantProductPools.unVerified.map((p) => _.pick(p, PICK)),
      ...pools.constantProductPools.verified.map((p) => _.pick(p, PICK)),
      // ...pools.stableSwapPools.unVerified.map((p) => _.pick(p, PICK)),
      ...pools.stableSwapPools.verified.map((p) => _.pick(p, PICK)),
    ];
  }, [pools]);
  /**
   * For each pool, fetch its
   * base and quote token amount
   * and update it in zustand store
   * (first call)
   */
  useEffect(() => {
    if (allPools.length && parachainApi) {
      allPools.forEach((pool) => {
        if (pool.poolId && pool.pair) {
          fetchAndUpdatePoolLiquidity(
            pool as any,
            setTokenAmountInLiquidityPool,
            parachainApi
          );
        }
      });
    }
  }, [allPools, parachainApi, setTokenAmountInLiquidityPool]);
  /**
   * Fetch and update LP Balances within
   * zustand store
   */
  useEffect(() => {
    if (allPools.length && selectedAccount && parachainApi) {
      allPools.forEach((pool) => {
        if (pool.poolId && pool.pair && pool.lpToken) {
          fetchBalanceByAssetId(
            parachainApi,
            selectedAccount.address,
            pool.lpToken
          ).then((lpBalance) => {
            setUserLpBalance(pool.poolId as number, lpBalance);
          });
        }
      });
    }
  }, [parachainApi, allPools, selectedAccount, setUserLpBalance]);
  /**
   * For each pool, update zustand
   * store with value of tokens
   * locked within them
   */
  useEffect(() => {
    if (allPools.length) {
      allPools.forEach((pool) => {
        if (pool.poolId && pool.pair) {
          let baseId = pool.pair.base.toString();
          let quoteId = pool.pair.quote.toString();

          if (apollo[baseId] && poolLiquidity[pool.poolId]) {
            const baseValue = new BigNumber(
              poolLiquidity[pool.poolId].tokenAmounts.baseAmount
            )
              .times(apollo[baseId])
              .toString();
            setTokenValueInLiquidityPool(pool.poolId, {
              baseValue,
            });
          }
          if (apollo[quoteId] && poolLiquidity[pool.poolId]) {
            const quoteValue = new BigNumber(
              poolLiquidity[pool.poolId].tokenAmounts.quoteAmount
            )
              .times(apollo[quoteId])
              .toString();
            setTokenValueInLiquidityPool(pool.poolId, {
              quoteValue,
            });
          }
        }
      });
    }
  }, [allPools, apollo, poolLiquidity, setTokenValueInLiquidityPool]);

  return null;
};

export default Updater;
