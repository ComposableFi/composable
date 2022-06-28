import { AssetId } from "@/defi/polkadot/types";
import { AmmId } from "@/defi/types";

export interface CreatePoolSlice {
    createPool: {
        currentStep: number;
        baseAsset: AssetId | "none";
        quoteAsset: AssetId | "none";
        ammId: AmmId | "none";
        swapFee: string;
        liquidity: {
            baseAmount: string;
            quoteAmount: string;
        };
        weights: {
            baseWeight: string;
            quoteWeight: string;
        };
        similarPool: {
            poolId: number;
            value: string;
            volume: string;
            fee: string;
        };
        setLiquidity: (liquidity: Partial<CreatePoolSlice["createPool"]["liquidity"]>) => void;
        setWeights: (weights: Partial<CreatePoolSlice["createPool"]["weights"]>) => void;
        setSimilarPool: (similarPool: Partial<CreatePoolSlice["createPool"]["similarPool"]>) => void;
        setSelectable: (items: Partial<{
            baseAsset: AssetId | "none";
            quoteAsset: AssetId | "none";
            ammId: AmmId | "none";
            swapFee: string;
            currentStep: number;
        }>) => void;
        resetSlice: () => void;
    }
}