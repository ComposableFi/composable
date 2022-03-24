// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { u128, u64, Struct, Vec, u8 } from '@polkadot/types-codec';

/** @name AssetsBalance */
export interface AssetsBalance extends u128 {}

/** @name CurrencyId */
export interface CurrencyId extends u128 {}

export type PHANTOM_ASSETS = 'assets';

/** @name Asset */
export interface Asset extends Struct {
    readonly name: Vec<u8>;
    readonly id: u64;
}