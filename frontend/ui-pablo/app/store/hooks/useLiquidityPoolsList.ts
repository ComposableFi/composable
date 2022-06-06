import { AssetMetadata, getAssetByOnChainId } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import useStore from "@/store/useStore";
import { DEFAULT_NETWORK_ID } from "@/updaters/constants";
import BigNumber from "bignumber.js";
import _ from "lodash";
import { useMemo } from "react";
import { DailyRewards } from "../poolStats/poolStats.types";
import { useAllLpTokenRewardingPools } from "./useAllLpTokenRewardingPools";

export interface LiquidityPoolRow {
  poolId: number;
  baseAsset: AssetMetadata;
  quoteAsset: AssetMetadata;
  totalVolume: BigNumber;
  apr: BigNumber;
  totalValueLocked: BigNumber;
  dailyRewards: DailyRewards[];
  lpTokenAssetId: string;
}

export const useLiquidityPoolsList = (): LiquidityPoolRow[] => {
  const { poolStats, poolStatsValue, poolLiquidity } = useStore();
  const allLpRewardingPools = useAllLpTokenRewardingPools();

  const liquidityPoolsList = useMemo(() => {
    return allLpRewardingPools.map((pool) => {
      const {pair,poolId} = pool;
      const baseAsset = getAssetByOnChainId(DEFAULT_NETWORK_ID, pair.base);
      const quoteAsset = getAssetByOnChainId(DEFAULT_NETWORK_ID, pair.quote);
      const lpTokenAssetId = pool.lpToken;

      let totalVolume = new BigNumber(0);
      if (poolStatsValue[pool.poolId]) {
        totalVolume = totalVolume.plus(poolStatsValue[pool.poolId].totalVolumeValue);
      }

      let totalValueLocked = new BigNumber(0);
      if (poolLiquidity[pool.poolId]) {
        const { baseValue, quoteValue } = poolLiquidity[pool.poolId].value;
        totalValueLocked = new BigNumber(baseValue).plus(quoteValue)
      }
      
      let dailyRewards: DailyRewards[] = [], apr = new BigNumber(0);
      if (poolStats[pool.poolId]) {
        dailyRewards = poolStats[pool.poolId].dailyRewards;
        apr = new BigNumber(poolStats[pool.poolId].apr)
      }

      return {
        poolId,
        baseAsset,
        quoteAsset,
        totalVolume,
        lpTokenAssetId,
        totalValueLocked,
        dailyRewards,
        apr
      };
    })

  }, [allLpRewardingPools.length, poolLiquidity, poolStatsValue, poolStats]);

  return liquidityPoolsList;
};
