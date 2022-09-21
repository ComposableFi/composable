import BigNumber from "bignumber.js";
import { useMemo } from "react";
import useStore from "../useStore";
/**
 * Get price from Apollo in USD
 * @param assetId asset id from chain e.g "1" for PICA
 * @returns BigNumber
 */
 export function useUSDPriceByAssetId(assetId: string | "none"): BigNumber {
    const {
        apollo
    } = useStore();

    if (apollo[assetId]) {
        return new BigNumber(apollo[assetId])
    }

    return new BigNumber(0)
}

export function useAssetBalance(chainId: string, assetId: string): BigNumber {
    const { assetBalances } = useStore();
    const balance = useMemo(() => {
        return new BigNumber(assetBalances?.[chainId]?.[assetId] || 0);
    }, [assetBalances, chainId, assetId])

    return balance;
}