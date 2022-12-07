import { Asset } from "shared";
import { useMemo } from "react";
import { TokenId } from "tokens";
import useStore from "@/store/useStore";

export function useAsset(assetId: string): Asset | undefined {
    const { substrateTokens } = useStore();
    const { tokens, hasFetchedTokens } = substrateTokens;

    const asset = useMemo(() => {
        if (!hasFetchedTokens) return;

        const tokenIds = Object.keys(tokens);
        for (const tokenId of tokenIds) {
            try {
                if (tokens[tokenId as TokenId].getPicassoAssetId() as string === assetId) {
                    return tokens[tokenId as TokenId];
                }
            } catch (err) {
                continue;
            }
        }
    }, [tokens, assetId, hasFetchedTokens]);

    return asset;
}