import { MockedAsset } from "@/store/assets/assets.types";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import { useMemo } from "react";

type AssetWithBalance = MockedAsset & { balance: BigNumber };

export function useAssetsWithBalance(networkId: string): AssetWithBalance[] {
    const {
        assetBalances,
        supportedAssets
    } = useStore();

    const assetsWithBalance = useMemo(() => {
        return supportedAssets.map(asset => {
            let balance = new BigNumber(0);
            if(assetBalances[networkId]?.[asset.network[networkId]]) {
                balance = new BigNumber(assetBalances[networkId][asset.network[networkId]])
            }

            return {
                ...asset,
                balance
            }
        }).filter(i => i.balance.gt(0))
    }, [assetBalances, supportedAssets, networkId]);

    return assetsWithBalance;
}