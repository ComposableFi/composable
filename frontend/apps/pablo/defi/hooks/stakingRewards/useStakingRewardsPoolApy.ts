import { calculateStakingRewardsPoolApy } from "@/defi/utils/stakingRewards";
import { useStakingRewardPool } from "@/store/stakingRewards/stakingRewards.slice";
import { useMemo } from "react";
import { useAverageLockTimeAndMultiplier } from "./useAverageMultiplierAndTime";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";

export function useStakingRewardsPoolApy(
  stakingRewardPoolId: string | undefined
): Record<string, BigNumber> {
  const stakingRewardPool = useStakingRewardPool(stakingRewardPoolId ?? "-");
  const { totalValueLocked } = useAverageLockTimeAndMultiplier();
  const { apollo } = useStore();

  const apy = useMemo(() => {
    if (!stakingRewardPool) return {};
    
    return Object.keys(stakingRewardPool.rewards).reduce((acc, curr) => {
      const rewardTokenValueUSD = new BigNumber(apollo[curr]) || new BigNumber(0);
      const { period, amount } = stakingRewardPool.rewards[curr].rewardRate;
      return {
        ...acc,
        [curr]: calculateStakingRewardsPoolApy(
          rewardTokenValueUSD,
          period === "PerSecond" ? amount.times(86400) : new BigNumber(0),
          totalValueLocked
        ),
      };
    }, {} as Record<string, BigNumber>);
  }, [stakingRewardPool, totalValueLocked, apollo]);

  return apy;
}
