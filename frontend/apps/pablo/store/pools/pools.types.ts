import { ConstantProductPool, LiquidityBootstrappingPool, StableSwapPool } from "@/defi/types";

export interface LiquidityBootstrappingPoolStats {
    startBalances: {
        quote: string;
        base: string;
    };
    currentBalances: {
        quote: string;
        base: string;
    };
    totalSold: string;
    totalRaised: string;
    totalVolume: string;
    liquidity: string;
}


export type LiquidityPoolType =
  | "StableSwap"
  | "ConstantProduct"
  | "LiquidityBootstrapping";


export interface PoolsSlice {
    pools: {
        constantProductPools: {
            verified: ConstantProductPool[];
            unVerified: ConstantProductPool[];
        },
        liquidityBootstrappingPools: {
            verified: LiquidityBootstrappingPool[];
            unVerified: LiquidityBootstrappingPool[];
            spotPrices: [number, string][]
        },
        stableSwapPools: {
            verified: StableSwapPool[];
            unVerified: StableSwapPool[];
        },
        setPoolsList: (
            pools: AnyPoolArray
        ) => void;
        setLiquidityBootstrappingPoolSpotPrice: (
            poolId: number,
            spotPrice: string
        ) => void;
    }
}

export type AnyPoolArray = Array<ConstantProductPool | LiquidityBootstrappingPool | StableSwapPool>
