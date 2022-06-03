import { AssetId } from "@/defi/polkadot/types";
import { ParachainId } from "substrate-react/dist/dotsama/types";
export interface AssetStore {
    assetId: AssetId;
    price: number;
    decimals: number;
    symbol: string;
    icon: string;
    balance: {
        [id in ParachainId]: string;
    }
}

export interface AssetsSlice {
    assets: {
        [assetId in AssetId]: AssetStore
    },
    updateAssetPrice: (
        assetId: AssetId,
        price: number
    ) => void,
    updateAssetBalance: (
        assetId: AssetId,
        parachainId: ParachainId,
        balance: string
    ) => void
}