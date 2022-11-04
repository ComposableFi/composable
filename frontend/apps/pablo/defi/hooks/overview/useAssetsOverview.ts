import { useCallback, useState } from "react";
import { useAssetsWithBalance } from "@/defi/hooks";
import { OwnedAsset } from "shared";
import { useParachainApi } from "substrate-react";
import { useAsyncEffect } from "@/hooks/useAsyncEffect";

export function useAssetsOverview(): OwnedAsset[] {
  const { parachainApi } = useParachainApi("picasso");
  const assetsWithBalance = useAssetsWithBalance();
  const [assetsWithValue, setAssetsWithValue] = useState<OwnedAsset[]>([]);

  const updateWithPrices = useCallback(async (): Promise<OwnedAsset[]> => {
    if (!parachainApi) return [];

    for (const asset of assetsWithBalance) {
      const price = await parachainApi.query.oracle.prices(
        asset.getPicassoAssetId() as string
      );
      asset.setPrice(price.toString());
    }

    return assetsWithBalance;
  }, [parachainApi, assetsWithBalance]);

  useAsyncEffect(async (): Promise<void> => {
    updateWithPrices().then(setAssetsWithValue);
  }, [updateWithPrices]);

  return assetsWithValue;
}
