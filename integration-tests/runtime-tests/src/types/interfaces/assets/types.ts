// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Bytes, Struct, u64 } from "@polkadot/types-codec";

/** @name Asset */
export interface Asset extends Struct {
  readonly name: Bytes;
  readonly id: u64;
}

export type PHANTOM_ASSETS = "assets";
