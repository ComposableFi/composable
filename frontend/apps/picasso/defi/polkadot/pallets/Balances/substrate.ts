import { getExistentialDeposit, toTokenUnitsBN } from "shared";
import { TokenId } from "tokens";
import { ApiPromise } from "@polkadot/api";
import { SUBSTRATE_NETWORKS } from "../../Networks";
import { SubstrateNetworkId } from "../../types";
import BigNumber from "bignumber.js";

export async function subscribeNativeBalance(
  account: string,
  api: ApiPromise | undefined,
  chainId: string,
  tokenId: TokenId,
  updateBalance: (data: {
    tokenId: TokenId;
    network: SubstrateNetworkId;
    balance: BigNumber;
    existentialDeposit: BigNumber;
  }) => void
) {
  if (!api) return;
  // create AccountId32 type byte array
  // and retrieve balances
  const accountId = api.createType("AccountId32", account);
  const subscription = await api.query.system.account(accountId, (result) => {
    const blObject: any = result.toJSON();

    const {
      data: { free },
    } = blObject;

    const { decimals } = SUBSTRATE_NETWORKS[chainId as SubstrateNetworkId];
    const bnBalance = toTokenUnitsBN(free, decimals);

    const existentialDeposit = getExistentialDeposit(api);

    updateBalance({
      network: chainId as SubstrateNetworkId,
      tokenId,
      balance: bnBalance,
      existentialDeposit,
    });
  });

  return subscription;
}
