import { ApiPromise } from "@polkadot/api";

export async function getClaimable(
  api: ApiPromise,
  collectionId: string,
  instanceId: string
) {
  const result = await api.rpc.stakingRewards.claimableAmount(
    collectionId,
    instanceId
  );

  return {
    result,
    collectionId,
    instanceId,
  };
}
