import { AssetId } from "@/defi/polkadot/types";

export interface RemoveLiquiditySlice {
    removeLiquidity: {
        poolId: number;
        baseAsset: AssetId | "none";
        quoteAsset: AssetId | "none";
        pooledAmountBase: string;
        pooledAmountQuote: string;
        setRemoveLiquidity: (
            stats: {
                poolId: number,
                baseAsset: AssetId,
                quoteAsset: AssetId,
                pooledAmountBase: string,
                pooledAmountQuote: string,
            }
        ) => void;
        resetRemoveLiquidity: () => void;
    }
}