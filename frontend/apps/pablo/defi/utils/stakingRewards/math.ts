import { StakingRewardPool } from "@/defi/types/stakingRewards";
import BigNumber from "bignumber.js";
import { RewardPoolRewardRatePeriod } from "@/defi/types/stakingRewards";

export function convertRewardRatePeriod(
  rewardRate: RewardPoolRewardRatePeriod
): BigNumber {
  return {
    PerSecond: new BigNumber(84600),
  }[rewardRate];
}

export function calculateRewardPerDayByAssetId(assetId: string, stakingRewardPool: StakingRewardPool | undefined): BigNumber {
    let rewardPerDay = new BigNumber(0);
    
    if (stakingRewardPool) {
        let rewardConfig = stakingRewardPool.rewards[assetId];

        if (rewardConfig) {
            rewardPerDay = rewardConfig.rewardRate.amount.times(
                convertRewardRatePeriod(rewardConfig.rewardRate.period)
            )
        }
    }

    return rewardPerDay;
}

export function calculateStakingRewardsPoolApy(
  rewardTokenValueInUSD: BigNumber,
  dailyRewardAmount: BigNumber,
  principalTokenValueInUSD: BigNumber,
  amountOfTokensStaked: BigNumber
): BigNumber {
  if (principalTokenValueInUSD.eq(0) || amountOfTokensStaked.eq(0)) { return new BigNumber(0) }

  let num = rewardTokenValueInUSD.times(dailyRewardAmount).times(365);
  let den = principalTokenValueInUSD.times(amountOfTokensStaked)
  return num.div(den);
}