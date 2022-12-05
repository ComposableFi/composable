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
          const picaId = asset.getPicassoAssetId() as string;
          return picaId === assetId;
        } catch (err: any) {
          console.log("Error Missing ID: ", asset.getSymbol(), err.message);
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
