import { StakingRewardPool } from "@/defi/types/stakingRewards";
import { ApiPromise } from "@polkadot/api";
import { decodeStakingRewardPool } from "./decode";

export async function fetchStakingRewardPools(
  parachainApi: ApiPromise,
  assetIds: Array<string>
): Promise<StakingRewardPool[]> {
  let _stakingRewardPools: StakingRewardPool[] = [];

  try {
    for (
      const assetId of assetIds
    ) {
      try {
        let stakingRewardPoolAtIndex: any =
          await parachainApi.query.stakingRewards.rewardPools(
            assetId
          );

        stakingRewardPoolAtIndex = stakingRewardPoolAtIndex.toJSON();

        if (stakingRewardPoolAtIndex == null) {
          throw new Error(`[AssetId: ${assetId}] Staking Reward Pool does not exist`);
        }

        const _decoded = decodeStakingRewardPool(
          stakingRewardPoolAtIndex
        );

        _stakingRewardPools.push(_decoded);
      } catch (err: any) {
        console.error(err.message);
      }
    }
  } catch (err) {
    console.error(err);
  }

  return _stakingRewardPools;
}
