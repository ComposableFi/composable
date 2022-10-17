// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Null, Struct, Text, u128 } from '@polkadot/types-codec';

/** @name AssetsBalance */
export interface AssetsBalance extends u128 {}

/** @name ComposableTraitsDefiCurrencyPairCurrencyId */
export interface ComposableTraitsDefiCurrencyPairCurrencyId extends Struct {
  readonly base: CurrencyId;
  readonly quote: CurrencyId;
}

/** @name ComposableTraitsDefiSellCurrencyId */
export interface ComposableTraitsDefiSellCurrencyId extends CurrencyId {}

/** @name ComposableTraitsXcmCumulusMethodId */
export interface ComposableTraitsXcmCumulusMethodId extends Null {}

/** @name ComposableTraitsXcmXcmSellRequest */
export interface ComposableTraitsXcmXcmSellRequest extends Null {}

/** @name CurrencyId */
export interface CurrencyId extends u128 {}

/** @name CustomRpcBalance */
export interface CustomRpcBalance extends SafeRpcWrapper {}

/** @name CustomRpcCurrencyId */
export interface CustomRpcCurrencyId extends SafeRpcWrapper {}

/** @name SafeRpcWrapper */
export interface SafeRpcWrapper extends Text {}

export type PHANTOM_COMMON = 'common';
