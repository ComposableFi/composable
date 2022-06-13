import { getAssetByOnChainId } from "@/defi/polkadot/Assets";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import _ from "lodash";
import { useEffect, useMemo } from "react";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "../constants";
import {
  fetchAndUpdatePoolLiquidity,
} from "./utils";
import { fetchBalanceByAssetId } from "../assets/utils";

const PICK = ["poolId", "pair", "lpToken"];
const Updater = () => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const {
    assets,
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
      ...pools.constantProductPools.unVerified.map((p) => _.pick(p, PICK)),
      ...pools.constantProductPools.verified.map((p) => _.pick(p, PICK)),
      ...pools.stableSwapPools.unVerified.map((p) => _.pick(p, PICK)),
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
  }, [allPools.length, parachainApi]);
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
            DEFAULT_NETWORK_ID,
            selectedAccount.address,
            pool.lpToken
          ).then((lpBalance) => {
            setUserLpBalance(pool.poolId as number, lpBalance);
          });
        }
      });
    }
  }, [parachainApi, allPools.length, selectedAccount]);
  /**
   * For each pool, update zustand
   * store with value of tokens
   * locked within them
   */
  useEffect(() => {
    if (allPools.length) {
      allPools.forEach((pool) => {
        if (pool.poolId && pool.pair) {
          const baseAssetMeta = getAssetByOnChainId(
            DEFAULT_NETWORK_ID,
            pool.pair.base
          );
          const quoteAssetMeta = getAssetByOnChainId(
            DEFAULT_NETWORK_ID,
            pool.pair.quote
          );

          if (assets[baseAssetMeta.assetId] && poolLiquidity[pool.poolId]) {
            const baseValue = new BigNumber(
              poolLiquidity[pool.poolId].tokenAmounts.baseAmount
            )
              .times(assets[baseAssetMeta.assetId].price)
              .toString();
            setTokenValueInLiquidityPool(pool.poolId, {
              baseValue,
            });
          }
          if (assets[quoteAssetMeta.assetId] && poolLiquidity[pool.poolId]) {
            const quoteValue = new BigNumber(
              poolLiquidity[pool.poolId].tokenAmounts.quoteAmount
            )
              .times(assets[quoteAssetMeta.assetId].price)
              .toString();
            setTokenValueInLiquidityPool(pool.poolId, {
              quoteValue,
            });
          }
        }
      });
    }
  }, [allPools.length, assets]);

  return null;
};

export default Updater;
