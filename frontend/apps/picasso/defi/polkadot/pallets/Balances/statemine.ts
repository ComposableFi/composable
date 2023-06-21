import { ApiPromise } from "@polkadot/api";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import BigNumber from "bignumber.js";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { fromChainIdUnit } from "shared";
import { TokenBalance } from "@/stores/defi/polkadot/balances/slice";

export function subscribeStatemineBalance(
  api: ApiPromise,
  address: string,
  asset: TokenMetadata,
  chainId: SubstrateNetworkId,
  callback: (value: TokenBalance) => void
) {
  const onChainId =
    asset.chainId[chainId] instanceof BigNumber
      ? asset.chainId[chainId]?.toString()
      : asset.chainId[chainId];
  if (onChainId) {
    return api.query.assets.account(onChainId, address, (assetAccount: any) => {
      assetAccount.isSome && asset.decimals[chainId] !== null
        ? callback({
            free: fromChainIdUnit(
              new BigNumber(assetAccount.toJSON().balance.toString()),
              asset.decimals[chainId]
            ),
            locked: new BigNumber(0),
          })
        : callback({
            free: new BigNumber(0),
            locked: new BigNumber(0),
          });
    });
  }
}
