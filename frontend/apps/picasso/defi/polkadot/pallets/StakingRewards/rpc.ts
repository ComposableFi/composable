import { ApiPromise } from "@polkadot/api";

export async function getClaimable(
  api: ApiPromise,
  collectionId: string,
  instanceId: string
) {
  try {
    return await api.rpc.stakingRewards.claimableAmount(
      collectionId,
      instanceId
    );
  } catch (e) {
    return null;
  }
}
