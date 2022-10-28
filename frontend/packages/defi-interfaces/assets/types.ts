// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Bytes, Struct, u32, u64 } from '@polkadot/types-codec';
import { ComposableTraitsAssetsXcmAssetLocation } from '../types';

/** @name Asset */
export interface Asset extends Struct {
  readonly name: Bytes;
  readonly id: u64;
  readonly decimals: u32;
  readonly foreignId: ComposableTraitsAssetsXcmAssetLocation
}

export type PHANTOM_ASSETS = 'assets';
