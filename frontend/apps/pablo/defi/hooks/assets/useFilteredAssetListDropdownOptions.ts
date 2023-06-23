import { AssetDropdownOptions } from "@/defi/types";
import useStore from "@/store/useStore";
import { useMemo } from "react";

export function useFilteredAssetListDropdownOptions(
  assetId: string
): AssetDropdownOptions {
  const { substrateTokens } = useStore();
  const { tokens, hasFetchedTokens } = substrateTokens;

  const assetOptions = useMemo(() => {
    if (!hasFetchedTokens) return [];

    return Object.values(tokens)
      .filter((asset) => {
        try {
          return (
            !!asset.getPicassoAssetId()?.toString() &&
            asset.getPicassoAssetId.toString() !== assetId
          );
        } catch (err: any) {
          return false;
        }
      })
      .map((asset) => ({
        value: asset.getPicassoAssetId() as string,
        label: asset.getName(),
        shortLabel: asset.getSymbol(),
        icon: asset.getIconUrl(),
      }));
  }, [assetId, tokens, hasFetchedTokens]);
  return assetOptions;
}