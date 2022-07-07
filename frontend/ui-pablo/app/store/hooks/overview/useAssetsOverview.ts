import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import { useMemo } from "react";
import { useAssetsWithBalance } from "@/defi/hooks";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";

export function useAssetsOverview(limit: number = 5) {
  const { apollo } = useStore();
  const assetsWithBalance = useAssetsWithBalance(DEFAULT_NETWORK_ID);

  const withBalance = useMemo(() => {
    return assetsWithBalance.filter(i => i.balance.gt(0)).slice(0, limit).map(asset => {
      let priceUsd = new BigNumber(0);
      if (apollo[asset.network[DEFAULT_NETWORK_ID]]) {
        priceUsd = new BigNumber(apollo[asset.network[DEFAULT_NETWORK_ID]])
      }
      return {
        ...asset,
        priceUsd
      }
    })
  }, [assetsWithBalance, apollo, limit]);

  return withBalance;
}