import { getAssetOnChainId } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import BigNumber from "bignumber.js";
import useStore from "../useStore";
/**
 * Get price from Apollo in USD
 * @param assetId string on chain asset id but in string
 * @returns string
 */
export function useAssetPrice(assetId: string): string {
    const {
        apollo
    } = useStore();
    if (apollo[assetId]) {
        return apollo[assetId]
    }
    return "0"
}

/**
 * Get price from Apollo in USD
 * @param assetId hardcoded asset ids on FE or "none"
 * @returns BigNumber
 */
 export function useUSDPriceByAssetId(assetId: AssetId | "none"): BigNumber {
    const {
        apollo
    } = useStore();

    if (assetId === "none") return new BigNumber(0)

    let assetOnChainId: number | null | string = getAssetOnChainId(DEFAULT_NETWORK_ID, assetId);

    if (assetOnChainId) {
        assetOnChainId = assetOnChainId.toString();
    
        if (apollo[assetOnChainId]) {
            return new BigNumber(apollo[assetOnChainId])
        }
    }

    return new BigNumber(0)
}