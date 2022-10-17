// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Struct, Vec } from '@polkadot/types-codec';
import type { AssetId } from '@polkadot/types/interfaces/runtime';

/** @name PalletCurrencyFactoryRanges */
export interface PalletCurrencyFactoryRanges extends Struct {
  readonly ranges: Vec<PalletCurrencyFactoryRangesRange>;
}

/** @name PalletCurrencyFactoryRangesRange */
export interface PalletCurrencyFactoryRangesRange extends Struct {
  readonly current: AssetId;
  readonly end: AssetId;
}

export type PHANTOM_CURRENCYFACTORY = 'currencyFactory';
