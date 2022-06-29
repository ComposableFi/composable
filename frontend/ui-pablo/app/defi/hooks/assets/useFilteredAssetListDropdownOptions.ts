import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { MockedAsset } from "@/store/assets/assets.types";
import useStore from "@/store/useStore";
import { useMemo } from "react";

export function useFilteredAssetListDropdownOptions(assetId: string): { value: string; label: string; shortLabel: string, icon: string }[] {
    const { supportedAssets } = useStore();

    const selectedAsset = useMemo(() => {
        return supportedAssets.filter(asset => asset.network[DEFAULT_NETWORK_ID] !== assetId).map((asset) => ({
            value: asset.network[DEFAULT_NETWORK_ID],
            label: asset.name,
            shortLabel: asset.symbol,
            icon: asset.icon,
          }));
    }, [supportedAssets, assetId]);

    return selectedAsset;
}