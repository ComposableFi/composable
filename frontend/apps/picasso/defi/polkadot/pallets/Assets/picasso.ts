import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";

export type PicassoRpcAsset = {
    name: string;
    id: BigNumber;
    decimals?: number;
}

export async function picassoAssetsList(api: ApiPromise): Promise<PicassoRpcAsset[]> {
    try {
        const assetsList = await api.rpc.assets.listAssets();
        return assetsList.map((asset) => {
            return {
                name: asset.name.toUtf8(),
                id: new BigNumber(asset.id.toString())
            }
        });
    } catch (err) {
        console.log('[picassoAssetsList] ', err);
        return [];
    }
}