import { TOKENS } from "@/defi/Tokens";
import { Token, TokenId } from "@/defi/types";
import { PicassoAssets } from "./PicassoAssets";

export function getTokenByOnChainId(onChainId: number): Token {
    return TOKENS[PicassoAssets[onChainId]];
}

export function getOnChainIdByTokenId(tokenId: TokenId): number | null {
    const token = Object.entries(PicassoAssets).find(([onChainId, _tokenId]) => {
        return tokenId === _tokenId
    }) as [number, TokenId][] | undefined;

    if (token && token.length) {
        return token[0][0];
    }

    return null;
}