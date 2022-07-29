import { PoolFeeConfig } from "./PoolFeeConfig";

export interface StableSwapPool {
    poolId: number;
    owner: string;
    pair: {
      base: number;
      quote: number;
    }
    lpToken: string;
    amplificationCoefficient: string;
    feeConfig: PoolFeeConfig;
}