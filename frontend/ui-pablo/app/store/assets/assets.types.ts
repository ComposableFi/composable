import { AssetId } from "@/defi/polkadot/types";
import { ParachainId, RelayChainId } from "substrate-react/dist/dotsama/types";
export interface AssetStore {
    assetId: AssetId;
    price: number;
    decimals: number;
    symbol: string;
    icon: string;
}

type AnyChain = ParachainId | RelayChainId
export interface AssetsSlice {
    assets: {
        [assetId in AssetId]: AssetStore
    },
    balances: {
        [id in AssetId]: {
            [id in AnyChain]: string
        }
    },
    apollo: {
        [id: string]: string;
    }
    updateApolloPrice: (
        assetId: string,
        price: string
    ) => void;
    updateAssetBalance: (
        assetId: AssetId,
        parachainId: ParachainId,
        balance: string
    ) => void;
}