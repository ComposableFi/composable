import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useEffect, useState } from "react";
import useStore from "../useStore";
import { useAllLpTokenRewardingPools } from "./useAllLpTokenRewardingPools";

export const useOnChainAssetIds = (): Set<string> => {
    const { supportedAssets } = useStore();

    const [assetIds, setAssetIds] = useState(new Set<string>());

    useEffect(() => {
        let assetIds = supportedAssets.map(i => {
            return i.network[DEFAULT_NETWORK_ID]
        }).filter(Boolean);
        setAssetIds(new Set(assetIds))
    }, [supportedAssets]);

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