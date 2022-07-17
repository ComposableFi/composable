import { MockedAsset } from "@/store/assets/assets.types";
import { DEFAULT_NETWORK_ID } from "../constants";

export function matchAssetByPicassoId(asset: MockedAsset, assetId: string): boolean {
    return !!asset.network[DEFAULT_NETWORK_ID] && asset.network[DEFAULT_NETWORK_ID] === assetId;
}