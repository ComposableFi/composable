import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { fromChainIdUnit, unwrapNumberOrHex } from "shared";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";

export type KusamaAsset = {
  chainId: string;
  decimals: number;
  name: string;
  symbol: string;
  existentialDeposit: BigNumber;
};

export function kusamaAssetsList(api: ApiPromise): Promise<KusamaAsset> {
  const existentialDeposit = api.consts.balances.existentialDeposit;
  return new Promise((res) => {
    res({
      chainId: "1",
      name: SUBSTRATE_NETWORKS.kusama.tokenId,
      decimals: SUBSTRATE_NETWORKS.kusama.decimals,
      symbol: SUBSTRATE_NETWORKS.kusama.symbol,
      existentialDeposit: fromChainIdUnit(
        unwrapNumberOrHex(existentialDeposit.toString()),
        SUBSTRATE_NETWORKS.kusama.decimals
      ),
    });
  });
}
