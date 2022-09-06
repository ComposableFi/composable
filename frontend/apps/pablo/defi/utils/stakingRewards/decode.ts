import {
  StakingRewardPool,
  StakingRewardPoolLockConfig,
} from "@/defi/types/stakingRewards";
import BigNumber from "bignumber.js";
import { fromChainUnits, fromPerbill } from "../units";

export function decodeStakingRewardPool(pool: any): StakingRewardPool {
  return {
    assetId: new BigNumber(pool.assetId),
    claimedShares: new BigNumber(pool.claimedShares),
    endBlock: new BigNumber(pool.endBlock),
    lock: {
      durationPresets: Object.keys(pool.lock.durationPresets).reduce(
        (acc, presetDuration) => {
          return {
            ...acc,
            [presetDuration]: fromPerbill(
              pool.lock.durationPresets[presetDuration]
            ),
          };
        },
        {} as StakingRewardPoolLockConfig["durationPresets"]
      ),
      unlockPenalty: fromPerbill(pool.lock.unlockPenalty),
    },
    owner: pool.owner,
    rewards: Object.keys(pool.rewards).reduce((acc, assetId) => {
      return {
        ...acc,
        [assetId]: {
          assetId: new BigNumber(pool.rewards[assetId].assetId),
          claimedRewards: new BigNumber(pool.rewards[assetId].claimedRewards),
          lastUpdatedTimestamp: pool.rewards[assetId].lastUpdatedTimestamp,
          maxRewards: new BigNumber(pool.rewards[assetId].maxRewards),
          rewardRate: {
            period: pool.rewards[assetId].rewardRate.period,
            amount: fromChainUnits(pool.rewards[assetId].rewardRate.amount),
          },
          totalDilutionAdjustment: new BigNumber(
            pool.rewards[assetId].totalDilutionAdjustment
          ),
          totalRewards: new BigNumber(pool.rewards[assetId].totalRewards),
        },
      };
    }, {} as StakingRewardPool["rewards"]),
    totalShares: fromChainUnits(pool.totalShares),
  };
}
