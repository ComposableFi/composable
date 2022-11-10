import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { getExistentialDeposit } from "shared";

export type PicassoRpcAsset = {
  name: string;
  id: BigNumber;
  decimals?: number;
};

export async function picassoAssetsList(
  api: ApiPromise
): Promise<PicassoRpcAsset[]> {
  try {
    const assetsList = await api.rpc.assets.listAssets();
    return assetsList.map(asset => {
      return {
        name: asset.name.toUtf8(),
        id: new BigNumber(asset.id.toString()),
        decimals: asset.decimals.toNumber(),
        foreignId: asset.foreignId,
        existentialDeposit: null
      };
    });
  } catch (err) {
    console.log("[picassoAssetsList] ", err);
    return [];
  }
}
