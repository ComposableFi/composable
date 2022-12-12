import { useEffect, useState } from "react";
import { useParachainApi } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import BigNumber from "bignumber.js";
import { Asset } from "shared";

export function useAssetIdTotalIssuance(
  assetId: BigNumber | string | undefined,
  name: string = "",
  symbol: string = "",
  iconUrl: string = ""
): BigNumber {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const [totalIssuance, setTotalIssuance] = useState(new BigNumber(0));

  useEffect(() => {
    if (!parachainApi || !assetId) return;

    new Asset(name, symbol, iconUrl).totalIssued().then(setTotalIssuance);
  }, [assetId, iconUrl, name, parachainApi, symbol]);

  return totalIssuance;
}
