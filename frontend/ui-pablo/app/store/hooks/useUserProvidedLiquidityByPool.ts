import { getAssetByOnChainId } from "@/defi/polkadot/Assets";
import { ConstantProductPool, StableSwapPool } from "@/store/pools/pools.types";
import useStore from "@/store/useStore";
import { processLiquidityTransactionsByAddress } from "@/updaters/liquidity/utils";
import { liquidityTransactionsByAddressAndPool } from "@/updaters/pools/subsquid";
import BigNumber from "bignumber.js";
import { useEffect, useMemo, useState } from "react";
import { useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "../../updaters/constants";
import { useAllLpTokenRewardingPools } from "./useAllLpTokenRewardingPools";

export const useUserProvidedLiquidityByPool = (
  poolId: number,
): {
  tokenAmounts: { baseAmount: BigNumber; quoteAmount: BigNumber };
  value: { baseValue: BigNumber; quoteValue: BigNumber };
} => {
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { assets, userProvidedLiquidity, setUserProvidedTokenAmountInPool } =
    useStore();

  const allPools = useAllLpTokenRewardingPools();
  const pool = useMemo<StableSwapPool | ConstantProductPool | undefined>(() => {
    return allPools.find(i => i.poolId === poolId)
  }, [undefined])

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
    if (pool && selectedAccount) {
      /**
       * this Updater effect should Move to view components or page load
       * For each pool query the liquidity
       * (amount of base and quote tokens)
       * provided by connectedAccount
       */
      if (pool && selectedAccount) {
        liquidityTransactionsByAddressAndPool(
          selectedAccount.address,
          pool.poolId
        ).then((userLiqTransactions) => {
          let { base, quote } = processLiquidityTransactionsByAddress(
            userLiqTransactions.data.pabloTransactions
          );
          setUserProvidedTokenAmountInPool((pool as any).poolId, {
            baseAmount: base.toString(),
            quoteAmount: quote.toString(),
          });
        });
      }
    }
  }, [pool, selectedAccount]);

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
