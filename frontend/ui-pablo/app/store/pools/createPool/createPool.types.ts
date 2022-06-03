import { AssetId } from "@/defi/polkadot/types";
import { AmmId } from "@/defi/types";
import { LiquidityPoolType } from "../pools.types";

export interface CreatePoolSlice {
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
    setLiquidity: (liquidity: Partial<CreatePoolSlice["liquidity"]>) => void;
    setWeights: (weights: Partial<CreatePoolSlice["weights"]>) => void;
    setSimilarPool: (similarPool: Partial<CreatePoolSlice["similarPool"]>) => void;
    setSelectable: (items: Partial<{
        baseAsset: AssetId | "none";
        quoteAsset: AssetId | "none";
        ammId: AmmId | "none";
        swapFee: string;
        currentStep: number;
    }>) => void;
    resetSlice: () => void;
}