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
    const { substrateTokens, substrateBalances } = useStore();
    const { tokenBalances } = substrateBalances;
    const { tokens, hasFetchedTokens } = substrateTokens;
    
    const tokenId = useMemo(() => {
        let tokenId: TokenId | null = null;
        if (!asset || !hasFetchedTokens) return tokenId;

        const tokenIds = Object.keys(tokens);
        for (const _tokenId of tokenIds) {
            try {
                const _tokenIdChainId = tokens[_tokenId as TokenId].getPicassoAssetId() as string;
                const _assetChainId = asset.getPicassoAssetId() as string;

                if (_tokenIdChainId === _assetChainId) {
                    tokenId = _tokenId as TokenId
                }
            } catch (err: any) {
                console.log(`useAssetBalance ${_tokenId} Error: `, err.message)
                continue;
            }
        }

        return tokenId
    }, [asset, tokens, hasFetchedTokens])

    const balance = useMemo(() => {
        if (!tokenId) return new BigNumber(0);

        return tokenBalances[network][tokenId].free;
    }, [network, tokenBalances, tokenId])

    return balance;
}