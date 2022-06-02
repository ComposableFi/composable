import { AssetId } from "@/defi/polkadot/types";
import useStore from "../useStore";

export function useAssetPrice(assetId: AssetId): number {
    const {
        assets
    } = useStore();
    if (assets[assetId]) {
        return assets[assetId].price
    } else {
        return 0;
    }
}