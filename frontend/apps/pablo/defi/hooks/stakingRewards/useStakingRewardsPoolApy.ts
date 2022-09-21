import { calculateStakingRewardsPoolApy } from "@/defi/utils/stakingRewards";
import { useStakingRewardPool } from "@/store/stakingRewards/stakingRewards.slice";
import BigNumber from "bignumber.js";
import { useMemo } from "react";

export function useStakingRewardsPoolApy(
    stakingRewardPoolId: string | undefined
): Record<string, BigNumber> {

    const stakingRewardPool = useStakingRewardPool(stakingRewardPoolId ?? "-");
    const apy = useMemo(() => {
        if (!stakingRewardPool) return {};
        const rewardTokenValueUSD = new BigNumber(10);


        return Object.keys(stakingRewardPool.rewards).reduce((acc, curr) => {
            const { period, amount } = stakingRewardPool.rewards[curr].rewardRate;
            const stakedAssetValueUSD = new BigNumber(10);
            const totalAssetStaked = new BigNumber(1_000_000);

            return {
                ...acc,
                [curr]: calculateStakingRewardsPoolApy(
                    rewardTokenValueUSD,
                    period === "PerSecond" ? amount.times(86400) : new BigNumber(0),
                    stakedAssetValueUSD,
                    totalAssetStaked
                )
            }
        }, {} as Record<string, BigNumber>);
        
    }, [stakingRewardPool])

    return apy;
}