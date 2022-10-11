import { fetchTotalIssued } from "@/defi/utils";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import _ from "lodash";
import { useEffect, useState } from "react";

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

    fetchTotalIssued(api, shareAssetId).then(setXTokensIssued);
  }, [api, shareAssetId]);

  return xTokensIssued;
}
