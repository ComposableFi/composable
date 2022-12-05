import { useStakingRewardPool } from "@/store/stakingRewards/stakingRewards.slice";
import { useState } from "react";
import { useAverageLockTimeAndMultiplier } from "./useAverageMultiplierAndTime";
import { useParachainApi } from "substrate-react";
import { calculateStakingRewardsPoolApy } from "@/defi/utils/stakingRewards";
import { useAsyncEffect } from "@/hooks/useAsyncEffect";
import BigNumber from "bignumber.js";

export function useStakingRewardsPoolApy(
  stakingRewardPoolId: string | undefined
): Record<string, BigNumber> {
  const stakingRewardPool = useStakingRewardPool(stakingRewardPoolId ?? "-");
  const { totalValueLocked } = useAverageLockTimeAndMultiplier();
  const { parachainApi } = useParachainApi("picasso");
  const [apy, setApy] = useState({});

  useAsyncEffect(async (): Promise<void> => {
    if (!stakingRewardPool || !parachainApi) return;

    let _apy: Record<string, BigNumber> = {};
    const rewards = Object.keys(stakingRewardPool.rewards);
    for (const rewardAsset of rewards) {
      const rewardAssetValue = await parachainApi.query.oracle.prices(rewardAsset);
        const { period, amount } = stakingRewardPool.rewards[rewardAsset].rewardRate;
        _apy[rewardAsset] = calculateStakingRewardsPoolApy(
        new BigNumber(rewardAssetValue.toString()),
        period === "PerSecond" ? amount.times(86400) : new BigNumber(0),
        totalValueLocked        
      )
    }

    setApy(_apy)
  }, [])

  return apy;
}
