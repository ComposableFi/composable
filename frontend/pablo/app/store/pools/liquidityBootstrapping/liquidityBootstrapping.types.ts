import { ParachainId } from "substrate-react/dist/dotsama/types";

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

export interface LBPoolSlice {
    liquidityBootstrappingPools: { list: LiquidityBootstrappingPool[]; }
    putLBPList: (
        lbPools: LiquidityBootstrappingPool[]
    ) => void;
    putLBPSpotPrice: (
        price: string,
        index: number
    ) => void;
}