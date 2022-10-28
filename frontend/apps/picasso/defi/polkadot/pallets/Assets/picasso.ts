import { ComposableTraitsXcmAssetsXcmAssetLocation } from "defi-interfaces";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";

export type PicassoRpcAsset = {
    name: string;
    id: BigNumber;
    decimals?: number;
    foreignId: ComposableTraitsXcmAssetsXcmAssetLocation
}

export async function picassoAssetsList(api: ApiPromise): Promise<PicassoRpcAsset[]> {
    try {
        const assetsList = await api.rpc.assets.listAssets();
        return assetsList.map((asset) => {
            return {
                name: asset.name.toUtf8(),
                id: new BigNumber(asset.id.toString()),
                decimals: asset.id.toNumber(),
                foreignId: asset.foreignId
            }
        });
    } catch (err) {
        console.log('[picassoAssetsList] ', err);
        return [];
    }
}