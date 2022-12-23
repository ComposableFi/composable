import { useCallback, useEffect, useState } from "react";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import { PoolConfig } from "@/store/pools/types";

export function useLiquidity(liquidityPool: PoolConfig | undefined): {
  baseAmount: BigNumber;
  quoteAmount: BigNumber;
} {
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
    const poolPair = liquidityPool.config.assets.map((a) =>
      a.getPicassoAssetId()?.toString()
    );
    const baseAsset = assets.find(
      (asset) => asset.getPicassoAssetId()?.toString() === poolPair[0]
    );
    const quoteAsset = assets.find(
      (asset) => asset.getPicassoAssetId()?.toString() === poolPair[1]
    );

    const accountId = liquidityPool.config.owner;
    if (baseAsset) baseAsset.balanceOf(accountId).then(setBaseAmount);
    if (quoteAsset) quoteAsset.balanceOf(accountId).then(setQuoteAmount);
  }, [liquidityPool, tokens, reset, hasFetchedTokens]);

  return { baseAmount, quoteAmount };
}
