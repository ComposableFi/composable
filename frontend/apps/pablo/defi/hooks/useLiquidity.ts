import { useCallback, useEffect, useState } from "react";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import { PoolConfig } from "@/store/pools/types";
import { useParachainApi } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { Asset, getSubAccount } from "shared";

export function useLiquidity(liquidityPool: PoolConfig | undefined | null): {
  baseAmount: BigNumber;
  quoteAmount: BigNumber;
  baseAsset: Asset | undefined;
  quoteAsset: Asset | undefined;
} {
  const { substrateTokens } = useStore();
  const { tokens, hasFetchedTokens } = substrateTokens;
  const [baseAmount, setBaseAmount] = useState(new BigNumber(0));
  const [quoteAmount, setQuoteAmount] = useState(new BigNumber(0));
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const [baseAsset, setBaseAsset] = useState<Asset | undefined>(undefined);
  const [quoteAsset, setQuoteAsset] = useState<Asset | undefined>(undefined);

  const reset = useCallback(() => {
    setBaseAmount(new BigNumber(0));
    setQuoteAmount(new BigNumber(0));
  }, []);

  useEffect(() => {
    if (!liquidityPool || !hasFetchedTokens || !parachainApi) {
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

    const accountId = getSubAccount(
      parachainApi,
      liquidityPool.poolId.toString()
    );
    if (baseAsset) baseAsset.balanceOf(accountId).then(setBaseAmount);
    if (quoteAsset) quoteAsset.balanceOf(accountId).then(setQuoteAmount);
    setBaseAsset(baseAsset);
    setQuoteAsset(quoteAsset);
  }, [liquidityPool, tokens, reset, hasFetchedTokens, parachainApi]);

  return { baseAmount, quoteAmount, baseAsset, quoteAsset };
}
