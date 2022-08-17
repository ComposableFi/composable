// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { CurrencyId } from '@composable/types/interfaces/common';
import type { Struct, bool, u128, u32 } from '@polkadot/types-codec';
import type { AccountId32 } from '@polkadot/types/interfaces/runtime';

/** @name ComposableTraitsBondedFinanceBondDuration */
export interface ComposableTraitsBondedFinanceBondDuration extends Struct {
  readonly Finite: {
    readonly returnIn: u32;
  } & Struct;
}

/** @name ComposableTraitsBondedFinanceBondOffer */
export interface ComposableTraitsBondedFinanceBondOffer extends Struct {
  readonly beneficiary: AccountId32;
  readonly asset: CurrencyId;
  readonly bondPrice: u128;
  readonly nbOfBonds: u128;
  readonly maturity: ComposableTraitsBondedFinanceBondDuration;
  readonly reward: ComposableTraitsBondedFinanceBondOfferReward;
  readonly keepAlive: bool;
}

/** @name ComposableTraitsBondedFinanceBondOfferReward */
export interface ComposableTraitsBondedFinanceBondOfferReward extends Struct {
  readonly asset: CurrencyId;
  readonly amount: u128;
  readonly maturity: u32;
}

export type PHANTOM_BONDEDFINANCE = 'bondedFinance';
