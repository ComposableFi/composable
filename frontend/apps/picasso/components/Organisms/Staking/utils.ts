import { RewardPool } from "@/stores/defi/polkadot/stakingRewards/slice";
import config from "@/constants/config";

export function getMaxDuration(
  hasRewardPools: boolean,
  picaRewardPool: RewardPool
) {
  return hasRewardPools
    ? Object.entries(picaRewardPool.lock.durationPresets).reduce(
        (a, [b, _]) => (a > Number(b) ? a : Number(b)),
        0
      )
    : 0;
}

export function getMinDuration(
  hasRewardPools: boolean,
  picaRewardPool: RewardPool
) {
  return hasRewardPools
    ? Object.entries(picaRewardPool.lock.durationPresets).reduce(
        (a, [b, _]) => (a < Number(b) ? a : Number(b)),
        0
      )
    : 0;
}

function getLabelFromDurationInSeconds(duration: string | number) {
  const value = Number(duration);
  if (value === 0) return "No Lock period";
  const SECONDS_IN_WEEK = 604800;
  const numberOfWeeks = value / SECONDS_IN_WEEK;
  const numberOfMonths = numberOfWeeks / 4;
  const andWeeks = numberOfWeeks % 4;
  let output = "";
  if (numberOfMonths === 0 && numberOfWeeks === 0) {
    output += "No lock period";
  } else if (Math.floor(numberOfMonths) === 0) {
    output += `${numberOfWeeks} weeks`;
  } else if (numberOfMonths === 1) {
    output += "1 month";
  } else if (Math.floor(numberOfMonths) !== numberOfMonths) {
    output += `${Math.floor(numberOfMonths)} months and ${andWeeks} weeks`;
  } else {
    output += `${numberOfMonths} months`;
  }

  return output;
}

export function getOptions(
  hasRewardPools: boolean,
  picaRewardPool: RewardPool
) {
  if (config.stakingRewards.demoMode)
    return config.stakingRewards.durationPresetOptions;
  return hasRewardPools
    ? Object.entries(picaRewardPool.lock.durationPresets).reduce(
        (acc, [duration, _]) => [
          ...acc,
          {
            label: getLabelFromDurationInSeconds(duration),
            value: Number(duration),
          },
        ],
        [] as any
      )
    : [];
}
