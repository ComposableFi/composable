import useStore from "@/store/useStore";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import BigNumber from "bignumber.js";
import _ from "lodash";
import { useMemo } from "react";
import { DailyRewards } from "../poolStats/poolStats.types";
import { useAllLpTokenRewardingPools } from "./useAllLpTokenRewardingPools";
import { MockedAsset } from "../assets/assets.types";
import { matchAssetByPicassoId } from "@/defi/utils";

export interface LiquidityPoolRow {
  poolId: number;
  baseAsset: MockedAsset | undefined;
  quoteAsset: MockedAsset | undefined;
  totalVolume: BigNumber;
  apr: BigNumber;
  totalValueLocked: BigNumber;
  dailyRewards: DailyRewards[];
  lpTokenAssetId: string;
}

export const useLiquidityPoolsList = (): LiquidityPoolRow[] => {
  const { poolStats, poolStatsValue, poolLiquidity, supportedAssets } = useStore();
  const allLpRewardingPools = useAllLpTokenRewardingPools();

  const liquidityPoolsList = useMemo(() => {
    return allLpRewardingPools.map((pool) => {
      const { poolId, pair } = pool;

      const baseAsset = supportedAssets.find(asset => matchAssetByPicassoId(asset, pair.base.toString()))
      const quoteAsset = supportedAssets.find(asset => matchAssetByPicassoId(asset, pair.quote.toString()))

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

  }, [allLpRewardingPools, poolLiquidity, poolStatsValue, poolStats, supportedAssets]);

  return liquidityPoolsList;
};