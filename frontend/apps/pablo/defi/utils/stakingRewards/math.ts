import { StakingRewardPool } from "@/defi/types/stakingRewards";
import { RewardPoolRewardRatePeriod } from "@/defi/types/stakingRewards";
import BigNumber from "bignumber.js";

export function convertRewardRatePeriod(
  rewardRate: RewardPoolRewardRatePeriod
): BigNumber {
  return {
    PerSecond: new BigNumber(84600),
  }[rewardRate];
}

export function calculateRewardPerDayByAssetId(
  assetId: string,
  stakingRewardPool: StakingRewardPool | undefined
): BigNumber {
  let rewardPerDay = new BigNumber(0);

  if (stakingRewardPool) {
    let rewardConfig = stakingRewardPool.rewards[assetId];

    if (rewardConfig) {
      rewardPerDay = rewardConfig.rewardRate.amount.times(
        convertRewardRatePeriod(rewardConfig.rewardRate.period)
      );
    }
  }

  return rewardPerDay;
}

export function calculateStakingRewardsPoolApy(
  rewardTokenValueInUSD: BigNumber,
  dailyRewardAmount: BigNumber,
  totalValueLocked: BigNumber
): BigNumber {
  if (totalValueLocked.eq(0)) {
    return new BigNumber(0);
  }
  let num = rewardTokenValueInUSD.times(dailyRewardAmount).times(365);
  return num.div(totalValueLocked);
}

export function calculateDurationPresetAPR(
  lockDurationInSeconds: BigNumber | undefined,
  rewardMultiplier: BigNumber
): BigNumber {
  if (!lockDurationInSeconds) {
    return new BigNumber(0);
  }

  const SECONDS_IN_YEAR = 31536000;
  const APR = rewardMultiplier.multipliedBy(
    SECONDS_IN_YEAR / Number(lockDurationInSeconds)
  );

  return APR;
}