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
            label: "",
            value: Number(duration),
          },
        ],
        [] as any
      )
    : [];
}
