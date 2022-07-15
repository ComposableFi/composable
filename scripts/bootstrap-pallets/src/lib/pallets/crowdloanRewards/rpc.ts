import { ApiPromise } from "@polkadot/api";

export const amountToClaim = async (api: ApiPromise, rewardAccount: Uint8Array) => {
  const availableClaim = await (api.rpc as any).crowdloanRewards.amountAvailableToClaimFor(rewardAccount);

  return availableClaim.toHuman();
};
