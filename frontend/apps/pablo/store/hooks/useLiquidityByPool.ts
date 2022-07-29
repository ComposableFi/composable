import {
  ConstantProductPool,
  LiquidityBootstrappingPool,
  StableSwapPool,
} from "@/defi/types";
import useStore from "@/store/useStore";
import { fetchAndUpdatePoolLiquidity } from "@/defi/utils";
import BigNumber from "bignumber.js";
import { useState, useEffect } from "react";
import { useParachainApi } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";

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
  value: {
    baseValue: BigNumber;
    quoteValue: BigNumber;
    totalValueLocked: BigNumber;
  };
} => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const { poolLiquidity, setTokenAmountInLiquidityPool } = useStore();
  const [baseAmount, setBaseAmount] = useState(new BigNumber(0));
  const [quoteAmount, setQuoteAmount] = useState(new BigNumber(0));
  /**
   * update store with latest
   * balance of both assets
   */
  useEffect(() => {
    if (pool && parachainApi) {
      fetchAndUpdatePoolLiquidity(pool as any, setTokenAmountInLiquidityPool, parachainApi)
    }
  }, [pool, parachainApi, setTokenAmountInLiquidityPool])
  /**
   * Use Updated balance of pool
   * from the zustand store
   */
  useEffect(() => {
    if (pool) {
      if (poolLiquidity[pool.poolId]) {
        setBaseAmount(
          new BigNumber(poolLiquidity[pool.poolId].tokenAmounts.baseAmount)
        );
        setQuoteAmount(
          new BigNumber(poolLiquidity[pool.poolId].tokenAmounts.quoteAmount)
        );
      }
    }
  }, [pool, poolLiquidity]);

  const [value, setValue] = useState({
    baseValue: new BigNumber(0),
    quoteValue: new BigNumber(0),
    totalValueLocked: new BigNumber(0),
  });
  /**
   * Pass down liquidity value (in USD) 
   * updated by
   * liquidity updater
   */
  useEffect(() => {
    if (pool) {
      if (poolLiquidity[pool.poolId]) {
        const baseValue = new BigNumber(poolLiquidity[pool.poolId].value.baseValue)
        const quoteValue = new BigNumber(poolLiquidity[pool.poolId].value.quoteValue) 
        
        setValue({
          baseValue: new BigNumber(poolLiquidity[pool.poolId].value.baseValue),
          quoteValue: new BigNumber(poolLiquidity[pool.poolId].value.quoteValue),
          totalValueLocked: baseValue.plus(quoteValue)
        });
      }
    }
  }, [pool, poolLiquidity]);

  return {
    tokenAmounts: {
      baseAmount,
      quoteAmount,
    },
    value,
  };
};
