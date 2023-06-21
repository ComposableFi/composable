import { Asset } from "shared";
import { ApiPromise } from "@polkadot/api";
import { useEffect, useState } from "react";
import BigNumber from "bignumber.js";
import { TokenId } from "tokens";

export type TotalXTokensIssuedProps = {
  shareAssetId?: string;
  api?: ApiPromise;
};

export function useTotalXTokensIssued({
  api,
  shareAssetId,
}: TotalXTokensIssuedProps): BigNumber {
  const [xTokensIssued, setXTokensIssued] = useState(new BigNumber(0));

  useEffect(() => {
    if (!api || !shareAssetId) return;

    const xAsset = new Asset("", "", "", "" as TokenId, api);
    xAsset.setIdOnChain("picasso", new BigNumber(shareAssetId));
    xAsset.totalIssued().then(setXTokensIssued);
  }, [api, shareAssetId]);

  return xTokensIssued;
}
