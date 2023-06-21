import {
  StakingRewardPool,
  StakingRewardPoolLockConfig,
} from "@/defi/types/stakingRewards";
import { Codec } from "@polkadot/types-codec/types";
import BigNumber from "bignumber.js";
import { fromChainUnits, fromPerbill } from "../units";

export function decodeStakingRewardPool(pool: any): StakingRewardPool {
  return {
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
    claimedShares: new BigNumber(pool.claimedShares),
    startBlock: new BigNumber(pool.startBlock),
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
    shareAssetId: new BigNumber(pool.shareAssetId).toString(),
    financialNftAssetId: new BigNumber(pool.financialNftAssetId).toString(),
    minimumStakingAmount: fromChainUnits(pool.minimumStakingAmount),
  };
}

export function decodeStake(stake: Codec): {
  fnftInstanceId: string,
  lock: {
    duration: BigNumber,
    startedAt: BigNumber,
    unlockPenalty: BigNumber
  },
  reductions: Record<string, BigNumber>,
  rewardPoolId: string,
  share: BigNumber,
  stake: BigNumber
} {
  const stakeItem: any = stake.toJSON();
  return {
    fnftInstanceId: new BigNumber(stakeItem.fnftInstanceId).toString(),
    lock: {
      duration: new BigNumber(stakeItem.lock.duration),
      startedAt: new BigNumber(stakeItem.lock.startedAt),
      unlockPenalty: fromPerbill(stakeItem.lock.unlockPenalty)
    },
    reductions: Object.keys(stakeItem.reductions).reduce((agg, cur) => {
      agg[cur] = new BigNumber(stakeItem.reductions[cur])
      return agg
    }, {} as Record<string, BigNumber>),
    rewardPoolId: new BigNumber(stakeItem.rewardPoolId).toString(),
    share: fromChainUnits(stakeItem.share),
    stake: fromChainUnits(stakeItem.stake)
  }
}