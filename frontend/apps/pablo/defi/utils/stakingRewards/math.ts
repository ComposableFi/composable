import { StakingRewardPool } from "@/defi/types/stakingRewards";
import BigNumber from "bignumber.js";
import { fromChainUnits } from "../units";
import { RewardPoolRewardRatePeriod } from "@/defi/types/stakingRewards";

export function convertRewardRatePeriod(
  rewardRate: RewardPoolRewardRatePeriod
): BigNumber {
  return {
    PerSecond: new BigNumber(84600),
  }[rewardRate];
}

export function calculateRewardPerDayByAssetId(assetId: string, stakingRewardPool: StakingRewardPool | null): BigNumber {
    let rewardPerDay = new BigNumber(0);
    
    if (stakingRewardPool) {
        let rewardConfig = stakingRewardPool.rewards[assetId];

        if (rewardConfig) {
            rewardPerDay = fromChainUnits(rewardConfig.rewardRate.amount).times(
                convertRewardRatePeriod(rewardConfig.rewardRate.period)
            )
        }
    }

    return rewardPerDay;
}