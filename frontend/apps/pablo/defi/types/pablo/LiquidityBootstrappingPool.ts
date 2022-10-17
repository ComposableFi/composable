import { PoolFeeConfig } from "./PoolFeeConfig";
import { ParachainId } from "substrate-react";

export interface LiquidityBootstrappingPool {
  id: string;
  poolId: number;
  owner: string;
  pair: {
    base: number;
    quote: number;
  };
  sale: {
    startBlock: string;
    endBlock: string;
    start: number;
    end: number;
    duration: number;
    initialWeight: number;
    finalWeight: number;
  };
  feeConfig: PoolFeeConfig;
  networkId: ParachainId;
  auctionDescription: string[];
}
