import { Asset } from "shared";
import { useMemo } from "react";
import { TokenId } from "tokens";
import useStore from "@/store/useStore";

export function useAsset(assetId: string): Asset | undefined {
    const { substrateTokens } = useStore();
    const { tokens, hasFetchedTokens } = substrateTokens;

    const asset = useMemo(() => {
        if (!hasFetchedTokens) return;

        for (const token in tokens) {
            if (tokens[token as TokenId].getPicassoAssetId() as string === assetId) {
                return tokens[token as TokenId];
            }
        }
    }, [tokens, assetId, hasFetchedTokens]);

    return asset;
}