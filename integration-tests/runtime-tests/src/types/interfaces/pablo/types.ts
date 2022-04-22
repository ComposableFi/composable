// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { ComposableTraitsDefiCurrencyPairCurrencyId } from '@composable/types/interfaces/common';
import type { Enum, Null, Struct } from '@polkadot/types-codec';
import type { AccountId32, Permill } from '@polkadot/types/interfaces/runtime';

/** @name ConstantProduct */
export interface ConstantProduct extends Struct {
  readonly owner: AccountId32;
  readonly pair: ComposableTraitsDefiCurrencyPairCurrencyId;
  readonly fee: Permill;
  readonly ownerFee: Permill;
}

/** @name LiquidityBootstrapping */
export interface LiquidityBootstrapping extends Null {}

/** @name PalletPabloPoolConfiguration */
export interface PalletPabloPoolConfiguration extends Enum {
  readonly isStableSwap: boolean;
  readonly asStableSwap: StableSwap;
  readonly isConstantProduct: boolean;
  readonly asConstantProduct: ConstantProduct;
  readonly isLiquidityBootstrapping: boolean;
  readonly asLiquidityBootstrapping: LiquidityBootstrapping;
  readonly type: 'StableSwap' | 'ConstantProduct' | 'LiquidityBootstrapping';
}

/** @name PalletPabloPoolInitConfiguration */
export interface PalletPabloPoolInitConfiguration extends PalletPabloPoolConfiguration {}

/** @name PalletPabloPriceCumulative */
export interface PalletPabloPriceCumulative extends Null {}

/** @name PalletPabloTimeWeightedAveragePrice */
export interface PalletPabloTimeWeightedAveragePrice extends Null {}

/** @name pool */
export interface pool extends PalletPabloPoolConfiguration {}

/** @name StableSwap */
export interface StableSwap extends Null {}

export type PHANTOM_PABLO = 'pablo';
