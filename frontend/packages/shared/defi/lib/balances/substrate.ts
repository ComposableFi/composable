import { SubstrateNetworkId, SUBSTRATE_NETWORKS, toTokenUnitsBN } from "shared";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { FrameSystemAccountInfo } from "defi-interfaces";

export async function subscribeNativeBalance(
  api: ApiPromise | undefined,
  account: string,
  chainId: SubstrateNetworkId,
  callback: (accountData: {
    locked: BigNumber;
    free: BigNumber;
  }) => void
) {
  if (!api) return;
  // create AccountId32 type byte array
  // and retrieve balances
  const accountId = api.createType("AccountId32", account);
  const subscription = await api.query.system.account(accountId, (result: FrameSystemAccountInfo) => {
    const { decimals } = SUBSTRATE_NETWORKS[chainId as SubstrateNetworkId];
    const bnBalance = toTokenUnitsBN(result.data.free.toString(), decimals);
    const bnLocked = toTokenUnitsBN(result.data.reserved.toString(), decimals);

    callback({
        free: bnBalance,
        locked: bnLocked
    })
  });

  return subscription;
}
