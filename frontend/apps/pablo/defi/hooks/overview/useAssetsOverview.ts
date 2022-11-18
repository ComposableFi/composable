import { useCallback, useState } from "react";
import { useAssetsWithBalance } from "@/defi/hooks";
import { OwnedAsset } from "shared";
import { useParachainApi } from "substrate-react";
import { useAsyncEffect } from "@/hooks/useAsyncEffect";
import { useOracleSlice } from "@/store/oracle/slice";
import BigNumber from "bignumber.js";

export function useAssetsOverview(): OwnedAsset[] {
  const { parachainApi } = useParachainApi("picasso");
  const oracleSlice = useOracleSlice();
  const assetsWithBalance = useAssetsWithBalance();
  const [assetsWithValue, setAssetsWithValue] = useState<OwnedAsset[]>([]);

  const updateWithPrices = useCallback(async (): Promise<OwnedAsset[]> => {
    if (!parachainApi) return [];

    for (const asset of assetsWithBalance) {
      try {
        asset.setPrice(oracleSlice.prices[asset.getSymbol()].coingecko.usd);
      } catch (err: any) {
        console.log('[useAssetsOverview]: ', err.message);
        asset.setPrice(new BigNumber(0));
      }
    }

    return assetsWithBalance;
  }, [parachainApi, assetsWithBalance]);

  useAsyncEffect(async (): Promise<void> => {
    updateWithPrices().then(setAssetsWithValue);
  }, [updateWithPrices]);

  return assetsWithValue;
}
