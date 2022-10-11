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
  lpTokenValueUSD: BigNumber,
  amountOfTokensStaked: BigNumber
): BigNumber {
  if (lpTokenValueUSD.eq(0) || amountOfTokensStaked.eq(0)) {
    return new BigNumber(0);
  }

  let num = rewardTokenValueInUSD.times(dailyRewardAmount).times(365);
  let den = lpTokenValueUSD.times(amountOfTokensStaked);
  return num.div(den);
}

export function calcualteDurationPresetAPR(
  lockDurationInSecods: BigNumber | undefined,
  rewardMultiplier: BigNumber
): BigNumber {
  if (!lockDurationInSecods) {
    return new BigNumber(0);
  }

  const SECONDS_IN_YEAR = 31536000;
  const APR = rewardMultiplier.multipliedBy(
    SECONDS_IN_YEAR / Number(lockDurationInSecods)
  );

  return APR;
}
