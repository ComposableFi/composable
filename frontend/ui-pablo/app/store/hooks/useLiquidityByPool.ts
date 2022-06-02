import { getAssetByOnChainId } from "@/defi/polkadot/Assets";
import {
  ConstantProductPool,
  LiquidityBootstrappingPool,
  StableSwapPool,
} from "@/store/pools/pools.types";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { useState, useEffect } from "react";
import { useParachainApi } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "../../updaters/constants";

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
  const { assets, liquidity } = useStore();
  const [baseAmount, setBaseAmount] = useState(new BigNumber(0));
  const [quoteAmount, setQuoteAmount] = useState(new BigNumber(0));
  const [value, setValue] = useState({
    baseValue: new BigNumber(0),
    quoteValue: new BigNumber(0),
    totalValueLocked: new BigNumber(0),
  });

  useEffect(() => {
    if (parachainApi && pool) {
      if (liquidity[pool.poolId]) {
        setBaseAmount(
          new BigNumber(liquidity[pool.poolId].tokenAmounts.baseAmount)
        );
        setQuoteAmount(
          new BigNumber(liquidity[pool.poolId].tokenAmounts.quoteAmount)
        );
      }
    }
  }, [pool]);

  useEffect(() => {
    if (pool) {
      if (liquidity[pool.poolId]) {
        const baseValue = new BigNumber(liquidity[pool.poolId].value.baseValue)
        const quoteValue = new BigNumber(liquidity[pool.poolId].value.quoteValue) 
        
        setValue({
          baseValue: new BigNumber(liquidity[pool.poolId].value.baseValue),
          quoteValue: new BigNumber(liquidity[pool.poolId].value.quoteValue),
          totalValueLocked: baseValue.plus(quoteValue)
        });
      }
    }
  }, [pool, assets, baseAmount]);

  return {
    tokenAmounts: {
      baseAmount,
      quoteAmount,
    },
    value,
  };
};
