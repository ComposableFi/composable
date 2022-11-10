import { ApiPromise } from "@polkadot/api";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import BigNumber from "bignumber.js";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { fromChainIdUnit } from "shared";

export function subscribeStatemineBalance(
  api: ApiPromise,
  address: string,
  asset: TokenMetadata,
  chainId: SubstrateNetworkId,
  callback: (value: BigNumber) => void
) {
  const onChainId = BigNumber.isBigNumber(asset.chainId[chainId])
    ? asset.chainId[chainId]?.toString()
    : asset.chainId[chainId];
  if (onChainId) {
    api.query.assets.account(onChainId, address, (assetAccount: any) => {
      assetAccount.isSome && asset.decimals[chainId] !== null
        ? callback(
            fromChainIdUnit(
              new BigNumber(assetAccount.toJSON().balance.toString()),
              asset.decimals[chainId]
            )
          )
        : new BigNumber(0);
    });
  }
}
