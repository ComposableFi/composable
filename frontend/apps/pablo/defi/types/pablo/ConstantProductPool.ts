import { PoolFeeConfig } from "./PoolFeeConfig";

export interface ConstantProductPool {
    poolId: number;
    owner: string;
    pair: {
      base: number;
      quote: number;
    }
    lpToken: string;
    feeConfig: PoolFeeConfig;
    baseWeight: string;
}