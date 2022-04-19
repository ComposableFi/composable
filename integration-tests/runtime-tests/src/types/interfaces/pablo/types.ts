// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Enum, Null } from '@polkadot/types-codec';

/** @name PalletPabloPoolConfiguration */
export interface PalletPabloPoolConfiguration extends Enum {
  readonly isStableSwap: boolean;
  readonly isConstantProduct: boolean;
  readonly isLiquidityBootstrapping: boolean;
  readonly type: 'StableSwap' | 'ConstantProduct' | 'LiquidityBootstrapping';
}

/** @name PalletPabloPoolInitConfiguration */
export interface PalletPabloPoolInitConfiguration extends PalletPabloPoolConfiguration {}

/** @name PalletPabloPriceCumulative */
export interface PalletPabloPriceCumulative extends Null {}

/** @name PalletPabloTimeWeightedAveragePrice */
export interface PalletPabloTimeWeightedAveragePrice extends Null {}

export type PHANTOM_PABLO = 'pablo';
