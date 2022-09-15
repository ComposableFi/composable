import { useEffect, useMemo } from "react";
import { useOwnedFinancialNfts } from "../financialNfts/financialNfts.slice";
import {
  useStakedPositions,
  useStakingRewardPool,
} from "../stakingRewards/stakingRewards.slice";

export interface StakingPositionsProps {
  stakedAssetId: string;
}

export function useStakingPositions({ stakedAssetId }: StakingPositionsProps) {
  const ownedFinancialNfts = useOwnedFinancialNfts();
  const stakingRewardPool = useStakingRewardPool(stakedAssetId);
  const stakingPositions = useStakedPositions(stakedAssetId);

  const financialNftCollectionId = useMemo(() => {
    if (!stakingRewardPool) return null;

    return stakingRewardPool.financialNftAssetId;
  }, [stakingRewardPool]);

  const userStakingEvents = useMemo(() => {
    if (!financialNftCollectionId) return [];

    return stakingPositions.filter(
      (position) => position.fnftCollectionId === financialNftCollectionId
    );
  }, [financialNftCollectionId, stakingPositions]);

  const currentlyOwnedFinancialNFts = useMemo(() => {
    if (userStakingEvents.length <= 0 || financialNftCollectionId === null)
      return [];
    if (ownedFinancialNfts[financialNftCollectionId] === undefined) return [];

    return userStakingEvents.filter((x) => {
      ownedFinancialNfts[financialNftCollectionId].includes(x.fnftInstanceId);
    });
  }, [userStakingEvents, ownedFinancialNfts, financialNftCollectionId]);

  return currentlyOwnedFinancialNFts;
}
