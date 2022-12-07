import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";

export type PicassoRpcAsset = {
  name: string;
  id: BigNumber;
  decimals?: number;
  existentialDeposit: BigNumber | null;
  ratio: AssetRatio | null;
};

type AssetRatio = {
  n: number;
  d: number;
};

export async function picassoAssetsList(
  api: ApiPromise
): Promise<PicassoRpcAsset[]> {
  try {
    const assetsList = await api.rpc.assets.listAssets();
    return assetsList.map((asset) => {
      return {
        name: asset.name.toUtf8(),
        id: new BigNumber(asset.id.toString()),
        decimals: asset.decimals.toNumber(),
        foreignId: asset.foreignId.toJSON(),
        existentialDeposit: new BigNumber(asset.existentialDeposit.toString()),
        ratio: (asset.ratio.toJSON() as AssetRatio) ?? null,
      };
    });
  } catch (err) {
    console.log("[picassoAssetsList] ", err);
    return [];
  }
}
