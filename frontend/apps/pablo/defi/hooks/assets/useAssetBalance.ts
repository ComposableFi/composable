import { useMemo } from "react";
import { LiquidityProviderToken, Asset } from "shared";
import { TokenId } from "tokens";
import { SubstrateNetwork } from "@/store/tokens/types";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";

export function useAssetBalance(
    asset: Asset | LiquidityProviderToken | undefined,
    network: SubstrateNetwork
): BigNumber {
    const { substrateTokens } = useStore();
    const { tokenBalances, tokens, hasFetchedTokens } = substrateTokens;
    
    const tokenId = useMemo(() => {
        let tokenId: TokenId | null = null;
        if (!asset || !hasFetchedTokens) return tokenId;

        for (const token in tokens) {
            try {
                const tokenOnChainId = tokens[token as TokenId].getPicassoAssetId() as string;
                if (tokenOnChainId === asset.getPicassoAssetId() as string) {
                    tokenId = token as TokenId
                }
            } catch (err: any) {
                continue;
            }
        }

    }, [asset, tokens, hasFetchedTokens])

    return tokenId ? tokenBalances[network][tokenId] : new BigNumber(0);
}