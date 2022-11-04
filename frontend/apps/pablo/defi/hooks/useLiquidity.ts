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
        const base = liquidityPool.getPair().getBaseAsset();
        const quote = liquidityPool.getPair().getQuoteAsset();

        const baseAsset = assets.find(asset => base.eq(
            asset.getPicassoAssetId(true) as BigNumber
        ));

        const quoteAsset = assets.find(asset => quote.eq(
            asset.getPicassoAssetId(true) as BigNumber
        ));

        const accountId = liquidityPool.getAccountId();
        if (baseAsset) baseAsset.balanceOf(accountId).then(setBaseAmount)
        if (quoteAsset) quoteAsset.balanceOf(accountId).then(setQuoteAmount)

    }, [liquidityPool, tokens, reset, hasFetchedTokens]);

    return { baseAmount, quoteAmount };
}