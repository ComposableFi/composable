// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Null, Struct, u128, u32 } from '@polkadot/types-codec';
import type { Balance, BlockNumber, Percent } from '@polkadot/types/interfaces/runtime';

/** @name ComposableTraitsOraclePrice */
export interface ComposableTraitsOraclePrice extends Struct {
  readonly price: u128;
  readonly block: BlockNumber;
}

/** @name ComposableTraitsOracleRewardTracker */
export interface ComposableTraitsOracleRewardTracker extends Null {}

/** @name PalletOracleAssetInfo */
export interface PalletOracleAssetInfo extends Struct {
  readonly threshold: Percent;
  readonly minAnswers: u32;
  readonly maxAnswers: u32;
  readonly blockInterval: BlockNumber;
  readonly rewardWeight: Balance;
  readonly slash: Balance;
}

/** @name PalletOraclePrePrice */
export interface PalletOraclePrePrice extends Null {}

/** @name PalletOraclePrice */
export interface PalletOraclePrice extends Null {}

/** @name PalletOracleWithdraw */
export interface PalletOracleWithdraw extends Struct {
  readonly stake: u128;
  readonly unlockBlock: u32;
}

export type PHANTOM_ORACLE = 'oracle';
