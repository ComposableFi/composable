import { matchAssetByPicassoId } from "@/defi/utils";
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