import { Asset } from "shared";
import { useMemo } from "react";
import useStore from "@/store/useStore";

export function useAssets(assetIds: string[]): Asset[] {
    const { substrateTokens } = useStore();
    const { tokens, hasFetchedTokens } = substrateTokens;

    const _assets = useMemo(() => {
        if (!hasFetchedTokens) return [];

        return Object.values(tokens).filter(asset => {
            try {
                const assetId = asset.getPicassoAssetId() as string
                return assetIds.includes(assetId)
            } catch (err: any) {
                console.log('Error: ', err.message);
            }
        });
    }, [assetIds, hasFetchedTokens, tokens]);

    return _assets;
}