import { DEFAULT_NETWORK_ID, matchAssetByPicassoId } from "@/defi/utils";
import { MockedAsset } from "@/store/assets/assets.types";
import useStore from "@/store/useStore";
import { useMemo } from "react";

export function useAsset(assetId: string): MockedAsset | undefined {
    const { supportedAssets } = useStore();

    const selectedAsset = useMemo(() => {
        return supportedAssets.find(asset => matchAssetByPicassoId(asset, assetId));
    }, [supportedAssets, assetId]);

    return selectedAsset;
}

export function useAssets(assetIds: string[]): MockedAsset[] {
    const { supportedAssets } = useStore();

    const selectedAsset = useMemo(() => {
        return supportedAssets.filter(asset => assetIds.includes(asset.network[DEFAULT_NETWORK_ID]));
    }, [supportedAssets, assetIds]);

    return selectedAsset;
}