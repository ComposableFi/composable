import { u128 } from "@polkadot/types";
import { Permill } from "@polkadot/types/interfaces/runtime";

export interface PabloPoolPair {
  base: u128;
  quote: u128;
}

export interface PabloPoolFeeConfig {
  feeRate: Permill;
  ownerFeeRate: Permill;
  protocolFeeRate: Permill;
}
