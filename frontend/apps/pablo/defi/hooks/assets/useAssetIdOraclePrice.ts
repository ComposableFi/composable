import { useEffect, useState } from "react";
import { useParachainApi } from "substrate-react";
import BigNumber from "bignumber.js";

export function useAssetIdOraclePrice(
  assetId: BigNumber | string | undefined
): BigNumber {
  const { parachainApi } = useParachainApi("picasso");
  const [assetPrice, setAssetPrice] = useState(new BigNumber(0));

  useEffect(() => {
    if (!assetId || !parachainApi) return;
    const _assetId = typeof assetId === "string" ? assetId : assetId.toString();

    try {
      parachainApi.query.oracle.prices(_assetId).then((price) => {
        setAssetPrice(new BigNumber(price.price.toString()));
      });
    } catch (err: any) {
      console.error('[useAssetIdOraclePrice] Error: ', err.message);
    }
  }, [assetId, parachainApi]);

  return assetPrice;
}
