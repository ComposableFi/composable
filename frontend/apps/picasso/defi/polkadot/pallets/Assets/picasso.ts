import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";

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
    return assetsList.map((asset) => {
      return {
        name: asset.name.toUtf8(),
        id: new BigNumber(asset.id.toString()),
        decimals:
          asset.name.toUtf8().toUpperCase() !== "USDT"
            ? asset.decimals.toNumber()
            : 6, // TODO: Temporary assign 6 decimals to USDT until RPC is ready
        foreignId: asset.foreignId,
        existentialDeposit: null,
      };
    });
  } catch (err) {
    console.log("[picassoAssetsList] ", err);
    return [];
  }
}
