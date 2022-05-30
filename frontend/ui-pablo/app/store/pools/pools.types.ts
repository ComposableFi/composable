import { ParachainId } from "substrate-react/dist/dotsama/types";
import { CreatePoolSlice } from "./createPool/createPool.types";

export interface ConstantProductPool {
    poolId: number;
    owner: string;
    pair: {
      base: number;
      quote: number;
    }
    lpToken: string;
    fee: number;
    ownerFee: number;
}

export interface StableSwapPool {
    poolId: number;
    owner: string;
    pair: {
      base: number;
      quote: number;
    }
    lpToken: string;
    amplificationCoefficient: string;
    fee: string;
    ownerFee: string;
}

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

export interface LiquidityBootstrappingPool {
    id: string;
    poolId: number;
    icon: string;
    owner: string,
    pair: {
        base: number;
        quote: number;
    },
    sale: {
        start: number;
        end: number;
        duration: number;
        initialWeight: number; // Percentages
        finalWeight: number; // Percentages
    }
    fee: string;
    spotPrice: string;
    networkId: ParachainId;
    auctionDescription: string[];
}

export type LiquidityPoolType =
  | "StableSwap"
  | "ConstantProduct"
  | "LiquidityBootstrapping";

export interface StableSwapPool {
    poolId: number;
    owner: string;
    pair: {
      base: number;
      quote: number;
    }
    lpToken: string;
    amplificationCoefficient: string;
    fee: string;
    ownerFee: string;
}

export interface LiquidityPoolsSlice {
    pools: {
        constantProductPools: {
            verified: ConstantProductPool[];
            nonVerified: ConstantProductPool[];
        },
        liquidityBootstrappingPools: {
            verified: LiquidityBootstrappingPool[];
            nonVerified: LiquidityBootstrappingPool[];
            spotPrices: [number, string][]
        },
        stableSwapPools: {
            verified: StableSwapPool[];
            nonVerified: StableSwapPool[];
        },
        createPool: CreatePoolSlice;
        user: {
            lpBalances: {
                [poolId: number]: string;
            }
            setUserLpBalance: (poolId: number, balance: string) => void;
        },
        setPoolsList: (
            pool: ConstantProductPool[] | LiquidityBootstrappingPool[] | StableSwapPool[],
            poolType: "StableSwap" | "ConstantProduct" | "LiquidityBootstrapping",
            verified: boolean
        ) => void;
        setLiquidityBootstrappingPoolSpotPrice: (
            poolId: number,
            spotPrice: string
        ) => void;
    }
}