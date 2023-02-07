import { StakingRewardPool } from "@/defi/types/stakingRewards";
import { ApiPromise } from "@polkadot/api";
import { decodeStakingRewardPool } from "./decode";

export async function fetchStakingRewardPools(
  parachainApi: ApiPromise,
  assetIds: Array<string>
): Promise<Array<{ pool: StakingRewardPool; assetId: string }>> {
  let _stakingRewardPools: Array<{ pool: StakingRewardPool; assetId: string }> = [];

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
          continue;
          // throw new Error(`[AssetId: ${assetId}] Staking Reward Pool does not exist`);
        }

        const _decoded = decodeStakingRewardPool(
          stakingRewardPoolAtIndex
        );

        _stakingRewardPools.push({ pool: _decoded, assetId });
      } catch (err: any) {
        console.error(err.message);
      }
    }
  } catch (err) {
    console.error(err);
  }

  return _stakingRewardPools;
}
