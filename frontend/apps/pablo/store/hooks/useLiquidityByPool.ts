import { useParachainApi } from "substrate-react";
import {
  ConstantProductPool,
  LiquidityBootstrappingPool,
  StableSwapPool,
} from "@/defi/types";
import {
  DEFAULT_NETWORK_ID,
  fetchPoolLiquidity,
} from "@/defi/utils";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { useState, useEffect } from "react";

export const useLiquidityByPool = (
  pool:
    | ConstantProductPool
    | StableSwapPool
    | LiquidityBootstrappingPool
    | undefined
): {
  tokenAmounts: {
    baseAmount: BigNumber;
    quoteAmount: BigNumber;
  };
} => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const { liquidityInPool, updatePoolLiquidity } = useStore();
  const [baseAmount, setBaseAmount] = useState(new BigNumber(0));
  const [quoteAmount, setQuoteAmount] = useState(new BigNumber(0));
  /**
   * update store with latest
   * balance of both assets
   */
  useEffect(() => {
    if (pool && parachainApi) {
      fetchPoolLiquidity(parachainApi, [pool]).then((liq) => {
        updatePoolLiquidity(
          pool.poolId.toString(),
          liq[pool.poolId.toString()]
        );
      });
    }
  }, [pool, parachainApi, updatePoolLiquidity]);
  /**
   * Use Updated balance of pool
   * from the zustand store
   */
  useEffect(() => {
    if (pool) {
      if (liquidityInPool[pool.poolId]) {
        setBaseAmount(liquidityInPool[pool.poolId].baseAmount);
        setQuoteAmount(liquidityInPool[pool.poolId].quoteAmount);
      }
    }
  }, [pool, liquidityInPool]);

  return {
    tokenAmounts: {
      baseAmount,
      quoteAmount,
    },
  };
};
