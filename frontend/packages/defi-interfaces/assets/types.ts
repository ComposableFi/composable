// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { CustomRpcBalance } from "../common";
import type {
  ComposableTraitsCurrencyRational64,
  XcmV1MultiLocation,
} from "../crowdloanRewards";
import type { Bytes, Option, Struct, u128, u32 } from "@polkadot/types-codec";

/** @name Asset */
export interface Asset extends Struct {
  readonly name: Bytes;
  readonly id: u128;
  readonly decimals: u32;
  readonly ratio: Option<ComposableTraitsCurrencyRational64>;
  readonly foreignId: Option<XcmV1MultiLocation>;
  readonly existentialDeposit: CustomRpcBalance;
}

export type PHANTOM_ASSETS = "assets";
