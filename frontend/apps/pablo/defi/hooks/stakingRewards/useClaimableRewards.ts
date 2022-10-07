import { MockedAsset } from "@/store/assets/assets.types";
import { useMemo } from "react";
import { useAssets } from "../assets";
import { useStakingPositions } from "./useStakingPositions";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import BigNumber from "bignumber.js";
import { Stake, StakingRewardPool } from "@/defi/types";

export interface ClaimableAsset extends MockedAsset {
  claimable: BigNumber;
}

type ClaimableRewardsProps = {
  stakedAssetId?: string;
};

function claimOfStake(
  stake: Stake,
  stakingRewardPool: StakingRewardPool,
  rewardAssetId: string
): BigNumber {
  if (stakingRewardPool.totalShares.eq(0)) {
    return new BigNumber(0);
  } else {
    let inflation = stake.reductions[rewardAssetId] || new BigNumber(0);

    return stakingRewardPool.rewards[rewardAssetId].totalRewards
      .times(stake.share)
      .div(stakingRewardPool.totalShares)
      .minus(inflation);
  }
}

function calculateClaim(
  stake: Stake,
  stakingRewardPool: StakingRewardPool,
  accountForPenalty: boolean = false
): [string, BigNumber][] {
  return Object.keys(stakingRewardPool.rewards).map((assetId) => {
    let claimable = claimOfStake(stake, stakingRewardPool, assetId);
    let is_penalized =
      stake.lock.startedAt.plus(stake.lock.duration).toNumber() > Date.now();

    if (!stakingRewardPool.rewards[assetId].totalRewards.eq(0)) {
      if (is_penalized && accountForPenalty) {
        claimable = claimable.minus(
          claimable.times(stakingRewardPool.lock.unlockPenalty)
        );
      }
    }

    claimable = BigNumber.min(
      claimable,
      stakingRewardPool.rewards[assetId].totalRewards.minus(
        stakingRewardPool.rewards[assetId].claimedRewards
      )
    );

    return [assetId, claimable];
  });
}

export function useClaimableRewards({
  stakedAssetId,
}: ClaimableRewardsProps): Array<ClaimableAsset> {
  const { stakingRewardPool, stakes } = useStakingPositions({
    stakedAssetId,
  });

  const rewardAssets = useAssets(
    stakingRewardPool ? Object.keys(stakingRewardPool.rewards) : []
  );

  const claimableAmounts = useMemo(() => {
    if (!stakingRewardPool) return [];

    return stakes.map((_stake) => {
      return calculateClaim(_stake, stakingRewardPool, false);
    });
  }, [stakes, stakingRewardPool]);

  return useMemo(() => {
    return rewardAssets.map((asset) => {
      const assetId = asset.network[DEFAULT_NETWORK_ID];
      let claimable = new BigNumber(0);
      if (claimableAmounts.length > 0) {
        const claimableFromStake = claimableAmounts.find(
          (claimableFromStake) => {
            return claimableFromStake.find(([id, val]) => id === assetId);
          }
        );

        if (claimableFromStake) {
          claimable = claimableFromStake[0][1];
        }
      }

      return { ...asset, claimable };
    });
  }, [rewardAssets, claimableAmounts]);
}
