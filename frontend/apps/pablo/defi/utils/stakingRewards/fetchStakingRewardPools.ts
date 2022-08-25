import { StakingRewardPool } from "@/defi/types/stakingRewards";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { decodeStakingRewardPool } from "./decode";

export async function fetchStakingRewardPools(
  parachainApi: ApiPromise
): Promise<StakingRewardPool[]> {
  let _stakingRewardPools: StakingRewardPool[] = [];

  try {
    const stakingRewardPoolCount =
      await parachainApi.query.stakingRewards.rewardPoolCount();
    for (
      let poolIndex = new BigNumber(1);
      poolIndex.lte(stakingRewardPoolCount.toHex());
      poolIndex = poolIndex.plus(1)
    ) {
      let stakingRewardPoolAtIndex =
        await parachainApi.query.stakingRewards.rewardPools(
          poolIndex.toString()
        );

      const _decoded = decodeStakingRewardPool(
        stakingRewardPoolAtIndex.toJSON(),
        poolIndex
      );
      _stakingRewardPools.push(_decoded);
    }
  } catch (err) {
    console.error(err);
  }

  return _stakingRewardPools;
}
