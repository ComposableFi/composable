import { Assets } from "@/defi/polkadot/Assets";
import { useEffect, useState } from "react";
import { useAllLpTokenRewardingPools } from "./useAllLpTokenRewardingPools";

export const useOnChainAssetIds = (): Set<string> => {
    const [assetIds, setAssetIds] = useState(new Set<string>());

    useEffect(() => {
        let defaultIds = new Set<string>();
        Object.values(Assets).forEach((asset) => {
            if (asset.supportedNetwork.picasso) {
                defaultIds.add(asset.supportedNetwork.picasso.toString());
            }
        });
        setAssetIds(defaultIds);
    }, []);

    const pools = useAllLpTokenRewardingPools();

    useEffect(() => {
        let lpAssetIds = new Set<string>();
        pools.forEach(pool => {
            if (!assetIds.has(pool.lpToken)) {
                lpAssetIds.add(pool.lpToken)
            }
        });

        if (lpAssetIds.size > 0) {
            let newSet = new Set([
                ...Array.from(lpAssetIds),
                ...Array.from(assetIds)
            ]);
            setAssetIds(newSet);
        }
    }, [pools, assetIds]);

    return assetIds;
}