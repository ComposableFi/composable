import { MockedAsset } from "@/store/assets/assets.types";
import { useMemo } from "react";
import { useAssets } from "../assets";
import { useStakingPositions } from "./useStakingPositions";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import BigNumber from "bignumber.js";

export interface ClaimableAsset extends MockedAsset {
    claimable: BigNumber;
}

type ClaimableRewardsProps = {
    stakedAssetId?: string;
}

export function useClaimableRewards({ stakedAssetId }: ClaimableRewardsProps): Array<ClaimableAsset> {
    const {
        stakingRewardPool,
        // ownedFinancialNftsHistory,
        // stakes,
    } = useStakingPositions({
        stakedAssetId
    });

    const rewardAssets = useAssets(
        stakingRewardPool ? Object.keys(stakingRewardPool.rewards) : []
    )

    return useMemo(() => {
        if (!stakingRewardPool) return [];

        const claimable = Object.keys(stakingRewardPool.rewards).map((curr) => {
            const asset = rewardAssets.find(_asset => _asset.network[DEFAULT_NETWORK_ID] === curr);

            if (asset) {
                let claimable = new BigNumber(0)
                return { ... asset, claimable };
            }

            return undefined
        })

        return claimable.filter(x => !!x) as ClaimableAsset[]
    }, [rewardAssets, stakingRewardPool])
}