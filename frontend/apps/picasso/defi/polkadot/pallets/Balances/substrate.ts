import { SubstrateNetworkId, toTokenUnitsBN } from "shared";
import { TokenId } from "tokens";
import { ApiPromise } from "@polkadot/api";
import { TokenBalance } from "@/stores/defi/polkadot/balances/slice";
import { SUBSTRATE_NETWORKS } from "shared/defi/constants";

export async function subscribeNativeBalance(
  account: string,
  api: ApiPromise | undefined,
  chainId: string,
  tokenId: TokenId,
  updateBalance: (data: {
    tokenId: TokenId;
    network: SubstrateNetworkId;
    balance: TokenBalance;
  }) => void
) {
  if (!api) return;
  // create AccountId32 type byte array
  // and retrieve balances
  const accountId = api.createType("AccountId32", account);
  const subscription = await api.query.system.account(accountId, (result) => {
    const blObject: any = result.toJSON();

    const {
      data: { free, reserved },
    } = blObject;

    const { decimals } = SUBSTRATE_NETWORKS[chainId as SubstrateNetworkId];
    const bnFree = toTokenUnitsBN(free, decimals);
    const bnLocked = toTokenUnitsBN(reserved, decimals);

    updateBalance({
      network: chainId as SubstrateNetworkId,
      tokenId,
      balance: {
        free: bnFree,
        locked: bnLocked,
      },
    });
  });

  return subscription;
}
