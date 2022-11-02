// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { XcmV1MultiLocation } from '@composable/types/interfaces/crowdloanRewards';
import type { Bytes, Option, Struct, u128, u32 } from '@polkadot/types-codec';

/** @name Asset */
export interface Asset extends Struct {
  readonly name: Bytes;
  readonly id: u128;
  readonly decimals: u32;
  readonly foreignId: Option<XcmV1MultiLocation>;
}

export type PHANTOM_ASSETS = 'assets';
