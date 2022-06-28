import { getAssetOnChainId } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import BigNumber from "bignumber.js";
import { useMemo } from "react";
import useStore from "@/store/useStore";

export const useAssetsWithBalance = () => {
    const { assets, balances, apollo } = useStore();

    const walletOverview = useMemo(() => {
      return Object.keys(assets).map((assetId) => {
        let onChainAssetId = getAssetOnChainId(DEFAULT_NETWORK_ID, assetId as AssetId);
        let price = new BigNumber(0), balance = new BigNumber(0)
        if (onChainAssetId && apollo[onChainAssetId.toString()]) {
          price = new BigNumber(apollo[onChainAssetId.toString()])
        }
        balance = new BigNumber(balances[assetId as AssetId].picasso)
  
        return {
          ...assets[assetId as AssetId],
          price,
          balance
        }
      }).filter(i => i.balance.gt(0))
    }, [assets, apollo, balances]);

    return walletOverview;
}