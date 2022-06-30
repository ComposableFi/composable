import { AssetId } from "@/defi/polkadot/types";
import { ParachainId } from "substrate-react/dist/dotsama/types";
import { CreatePoolSlice } from "../createPool/createPool.types";

export interface ConstantProductPool {
    poolId: number;
    owner: string;
    pair: {
      base: number;
      quote: number;
    }
    lpToken: string;
    feeConfig: {
        feeRate: string;
        ownerFeeRate: string;
        protocolFeeRate: string;
    }
    baseWeight: string;
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
    feeConfig: {
        feeRate: string;
        ownerFeeRate: string;
        protocolFeeRate: string;
    }
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
        startBlock: string;
        endBlock: string;
        start: number;
        end: number;
        duration: number;
        initialWeight: number; // Percentages
        finalWeight: number; // Percentages
    }
    feeConfig: {
        feeRate: string;
        ownerFeeRate: string;
        protocolFeeRate: string;
    }
    spotPrice: string;
    networkId: ParachainId;
    auctionDescription: string[];
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

export type AnyPoolArray = ConstantProductPool[] | StableSwapPool[] | LiquidityBootstrappingPool[]
