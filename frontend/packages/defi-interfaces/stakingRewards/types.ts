import { Enum } from "@polkadot/types";

export interface ClaimableAmountError extends Enum {
  readonly isArithmetic: boolean;
  readonly isStakeNotFound: boolean;
  readonly isRewardsPoolNotFound: boolean;

  readonly type: "Arithmetic" | "StakeNotFound" | "RewardsPoolNotFound";
}
