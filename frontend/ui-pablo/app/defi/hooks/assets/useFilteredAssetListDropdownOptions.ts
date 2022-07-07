import { DEFAULT_NETWORK_ID, matchAssetByPicassoId } from "@/defi/utils";
import useStore from "@/store/useStore";
import { useMemo } from "react";

export function useFilteredAssetListDropdownOptions(assetId: string) {
    const { supportedAssets } = useStore();

    const assetOptions = useMemo(() => {
        return supportedAssets.filter(asset => !matchAssetByPicassoId(asset, assetId)).map((asset) => ({
            value: asset.network[DEFAULT_NETWORK_ID],
            label: asset.name,
            shortLabel: asset.symbol,
            icon: asset.icon,
          }));
    }, [supportedAssets, assetId]);

    return assetOptions;
}