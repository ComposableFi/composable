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
 * @param assetId string on chain asset id but in string
 * @returns string
 */
 export function useUSDPriceByAssetId(assetId: AssetId | "none"): BigNumber {
    const {
        apollo
    } = useStore();

    if (assetId === "none") return new BigNumber(0)

    const assetOnChainId = getAssetOnChainId(DEFAULT_NETWORK_ID, assetId);

    if (assetOnChainId && apollo[assetOnChainId.toString()]) {
        return new BigNumber(apollo[assetId])
    }
    return new BigNumber(apollo[assetId])
}