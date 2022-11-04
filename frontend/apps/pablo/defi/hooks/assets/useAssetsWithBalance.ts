import { useMemo } from "react";
import { OwnedAsset } from "shared";
import useStore from "@/store/useStore";
import { TokenId } from "tokens";

export function useAssetsWithBalance(): OwnedAsset[] {
    const {
        substrateTokens
    } = useStore();
    const { tokens, tokenBalances, hasFetchedTokens } = substrateTokens;
    const { picasso } = tokenBalances;

    const assetsWithBalance = useMemo(() => {
        if (!hasFetchedTokens) return [];
        
        
        const tokenIds: TokenId[] = [];
        for (const token in picasso) {
            if (picasso[token as TokenId].gt(0)) {
                tokenIds.push(token as TokenId);
            }
        }

        let assetsWithBalance: OwnedAsset[] = [];
        for (const token of tokenIds) {
            assetsWithBalance.push(
                OwnedAsset.fromAsset(
                    tokens[token as TokenId],
                    picasso[token as TokenId]
                )
            )
        }

        return assetsWithBalance;
    }, [hasFetchedTokens, picasso, tokens]);

    return assetsWithBalance;
}