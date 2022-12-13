import { useCallback, useEffect, useState } from "react";
import { BasePabloPool } from "shared";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";

export function useLiquidity(
    liquidityPool: BasePabloPool | undefined
): { baseAmount: BigNumber; quoteAmount: BigNumber } {
    const { substrateTokens } = useStore();
    const { tokens, hasFetchedTokens } = substrateTokens;
    const [baseAmount, setBaseAmount] = useState(new BigNumber(0));
    const [quoteAmount, setQuoteAmount] = useState(new BigNumber(0));

    const reset = useCallback(() => {
        setBaseAmount(new BigNumber(0));
        setQuoteAmount(new BigNumber(0));
    }, []);

    useEffect(() => {
        if (!liquidityPool || !hasFetchedTokens) {
            reset();
            return;
        }

        const assets = Object.values(tokens);
        const poolPair = Object.keys(liquidityPool.getAssets().assets);
        const baseAsset = assets.find(asset => (asset.getPicassoAssetId(true) as BigNumber).eq(poolPair[0]))
        const quoteAsset = assets.find(asset => (asset.getPicassoAssetId(true) as BigNumber).eq(poolPair[1]))

        const accountId = liquidityPool.getAccountId();
        if (baseAsset) baseAsset.balanceOf(accountId).then(setBaseAmount)
        if (quoteAsset) quoteAsset.balanceOf(accountId).then(setQuoteAmount)

    }, [liquidityPool, tokens, reset, hasFetchedTokens]);

    return { baseAmount, quoteAmount };
}