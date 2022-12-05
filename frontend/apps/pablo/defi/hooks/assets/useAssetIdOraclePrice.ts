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

    parachainApi.query.oracle.prices(_assetId).then((price) => {
      setAssetPrice(new BigNumber(price.price.toString()));
    });
  }, [assetId, parachainApi]);

  return assetPrice;
}
