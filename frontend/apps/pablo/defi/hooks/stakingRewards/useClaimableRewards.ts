import BigNumber from "bignumber.js";
import { useMemo } from "react";
import { useAssetIdTotalIssuance, useAssets } from "../assets";
import { useStakingPositions } from "./useStakingPositions";
import { Stake, StakingRewardPool } from "@/defi/types";
import { ClaimableAsset, fromChainIdUnit } from "shared";

type ClaimableRewardsProps = {
  stakedAssetId?: string;
};

function claimOfStake(
  stake: Stake,
  stakingRewardPool: StakingRewardPool,
  rewardAssetId: string,
  xTokenTotalIssuance: BigNumber
): BigNumber {
  if (xTokenTotalIssuance.eq(0)) {
    return new BigNumber(0);
  } else {
    let inflation =
      fromChainIdUnit(stake.reductions[rewardAssetId]) || new BigNumber(0);

    const totalRewards = fromChainIdUnit(
      stakingRewardPool.rewards[rewardAssetId].totalRewards
    );
    const share = stake.share;
    const totalShares = xTokenTotalIssuance;
    const myShare = totalRewards.times(share).div(totalShares);

    return myShare.minus(inflation);
  }
}

function calculateClaim(
  stake: Stake,
  stakingRewardPool: StakingRewardPool,
  xTokenTotalIssuance: BigNumber,
  accountForPenalty: boolean = false
): [string, BigNumber, string][] {
  return Object.keys(stakingRewardPool.rewards).map((assetId) => {
    let claimable = claimOfStake(stake, stakingRewardPool, assetId, xTokenTotalIssuance);

    if (claimable.lte(0)) {
      claimable = new BigNumber(0);
    }

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
      fromChainIdUnit(
        stakingRewardPool.rewards[assetId].totalRewards.minus(
          stakingRewardPool.rewards[assetId].claimedRewards
        )
      )
    );

    return [assetId, claimable, stake.fnftInstanceId];
  });
}

export function useClaimableRewards({
  stakedAssetId,
}: ClaimableRewardsProps): {
  claimableAssets: Array<ClaimableAsset>;
  financialNftInstanceId: string;
} {
  const { stakingRewardPool, stakes } = useStakingPositions({
    stakedAssetId,
  });

  const totalIssued = useAssetIdTotalIssuance(
    stakingRewardPool?.shareAssetId
  );

  const rewardAssets = useAssets(
    stakingRewardPool ? Object.keys(stakingRewardPool.rewards) : []
  );

  const claimableAmounts = useMemo(() => {
    if (!stakingRewardPool || stakes.length === 0) return [];

    return calculateClaim(stakes[0], stakingRewardPool, totalIssued, false);
  }, [stakes, stakingRewardPool, totalIssued]);

  return useMemo(() => {
    let financialNftInstanceId = "-";
    const claimableAssets = rewardAssets.map((asset) => {
      const assetId = asset.getPicassoAssetId() as string;
      let claimableAmount = new BigNumber(0);
      const claimableAsset = ClaimableAsset.fromAsset(asset, claimableAmount);
      if (claimableAmounts.length > 0) {
        const claimableFromStake = claimableAmounts.find(
          ([_assetId, _val]) => _assetId === assetId
        );

        if (claimableFromStake) {
          financialNftInstanceId = claimableFromStake[2];
          claimableAsset.setClaimable(claimableFromStake[1]);
        }
      }

      return claimableAsset;
    });

    return { claimableAssets, financialNftInstanceId };
  }, [rewardAssets, claimableAmounts]);
}
