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
            try {
                if (tokens[token as TokenId].getPicassoAssetId() as string === assetId) {
                    return tokens[token as TokenId];
                }
            } catch (err) {
                console.log('Error:', err);
                return undefined;
            }
        }
    }, [tokens, assetId, hasFetchedTokens]);

    return asset;
}