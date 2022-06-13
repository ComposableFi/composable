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