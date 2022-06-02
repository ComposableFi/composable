import { AssetMetadata, getAssetByOnChainId } from "@/defi/polkadot/Assets";
import { ConstantProductPool, StableSwapPool } from "@/store/pools/pools.types";
import BigNumber from "bignumber.js";
import { useState, useEffect, useMemo } from "react";
import useStore from "@/store/useStore";
import { useSelectedAccount, useParachainApi } from "substrate-react";
import { fetchBalanceByAssetId } from "../../updaters/balances/utils";
import { DEFAULT_NETWORK_ID } from "../../updaters/constants";
import { useUserProvidedLiquidity } from "./useUserProvidedLiquidity";
import { useAllLpTokenRewardingPools } from "./useAllLpTokenRewardingPools";
import { useLiquidityByPool } from "./useLiquidityByPool";

export const usePoolDetails = (poolId: number) => {
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const { poolStats } = useStore();


  const allLpRewardingPools = useAllLpTokenRewardingPools();
  const [pool, setPool] =
    useState<StableSwapPool | ConstantProductPool | undefined>(undefined);

  const tokensLocked = useLiquidityByPool(pool);
  const liquidityProvided = useUserProvidedLiquidity(pool);

  const [baseAsset, setBaseAsset] =
    useState<AssetMetadata | undefined>(undefined);
  const [quoteAsset, setQuoteAsset] =
    useState<AssetMetadata | undefined>(undefined);
  const [lpBalance, setLpBalance] = useState(new BigNumber(0));

  useEffect(() => {
    let pool: StableSwapPool | ConstantProductPool | undefined =
      allLpRewardingPools.find((p) => p.poolId === poolId);

    if (pool) {
      setPool(pool);
      const base = getAssetByOnChainId("picasso", pool.pair.base);
      const quote = getAssetByOnChainId("picasso", pool.pair.quote);

      if (base) {
        setBaseAsset(base);
      }
      if (quote) {
        setQuoteAsset(quote);
      }
    } else {
      setPool(undefined);
      setBaseAsset(undefined);
      setQuoteAsset(undefined);
      setLpBalance(new BigNumber(0));
    }
  }, [poolId]);

  useEffect(() => {
    if (parachainApi && selectedAccount && pool) {
      fetchBalanceByAssetId(
        parachainApi,
        DEFAULT_NETWORK_ID,
        selectedAccount.address,
        pool.lpToken
      ).then((lpBalance) => {
        setLpBalance(new BigNumber(lpBalance));
      });
    }
  }, [parachainApi, selectedAccount, pool]);

  const _poolStats = useMemo(() => {
    if (poolStats[poolId]) {
      return poolStats[poolId];
    } else {
      return {
        totalVolume: "0",
        totalValueLocked: "0",
        apr: "0",
        _24HrFee: "0",
        _24HrVolume: "0",
        _24HrTransactionCount: 0,
        dailyRewards: [],
        _24HrFeeValue: "0",
        _24HrVolumeValue: "0",
        totalVolumeValue: "0"
      };
    }
  }, [poolStats, poolId]);

  return {
    baseAsset,
    quoteAsset,
    pool,
    lpBalance,
    liquidityProvided,
    tokensLocked,
    poolStats: _poolStats,
  };
};
