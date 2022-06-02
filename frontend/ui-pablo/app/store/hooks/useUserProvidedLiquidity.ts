import { getAssetByOnChainId } from "@/defi/polkadot/Assets";
import { ConstantProductPool, StableSwapPool } from "@/store/pools/pools.types";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { useEffect, useState } from "react";
import { DEFAULT_NETWORK_ID } from "../../updaters/constants";

export const useUserProvidedLiquidity = (
  pool: ConstantProductPool | StableSwapPool | undefined
): {
  tokenAmounts: { baseAmount: BigNumber; quoteAmount: BigNumber };
  value: { baseValue: BigNumber; quoteValue: BigNumber };
} => {
  const { assets, userProvidedLiquidity } = useStore();

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
