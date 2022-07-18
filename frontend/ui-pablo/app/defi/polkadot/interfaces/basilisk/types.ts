// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Bytes, Compact, Enum, Struct, Vec, bool, i128, u32, u64, u8 } from '@polkadot/types';
import type { AccountId, AssetId, Balance, BlockNumber, Hash } from '@polkadot/types/interfaces/runtime';
import type { AssetInstanceV1, MultiAssetV1, MultiLocationV1, XcmOrderV1, XcmV1 } from '@polkadot/types/interfaces/xcm';
import type { ITuple } from '@polkadot/types/types';

/** @name Address */
export interface Address extends AccountId {}

/** @name Amount */
export interface Amount extends i128 {}

/** @name AmountOf */
export interface AmountOf extends Amount {}

/** @name AssetDetails */
export interface AssetDetails extends Struct {
  readonly name: Bytes;
  readonly asset_type: AssetType;
  readonly existential_deposit: Balance;
  readonly locked: bool;
}

/** @name AssetDetailsT */
export interface AssetDetailsT extends AssetDetails {}

/** @name AssetInstance */
export interface AssetInstance extends AssetInstanceV1 {}

/** @name AssetMetadata */
export interface AssetMetadata extends Struct {
  readonly symbol: Bytes;
  readonly decimals: u8;
}

/** @name AssetNativeLocation */
export interface AssetNativeLocation extends MultiLocation {}

/** @name AssetPair */
export interface AssetPair extends Struct {
  readonly asset_in: AssetId;
  readonly asset_out: AssetId;
}

/** @name AssetType */
export interface AssetType extends Enum {
  readonly isToken: boolean;
  readonly isPoolShare: boolean;
  readonly asPoolShare: ITuple<[AssetId, AssetId]>;
}

/** @name BalanceInfo */
export interface BalanceInfo extends Struct {
  readonly amount: Balance;
  readonly assetId: AssetId;
}

/** @name BalanceOf */
export interface BalanceOf extends Balance {}

/** @name Chain */
export interface Chain extends Struct {
  readonly genesisHash: Bytes;
  readonly lastBlockHash: Bytes;
}

/** @name ClassData */
export interface ClassData extends Struct {
  readonly is_pool: bool;
}

/** @name ClassId */
export interface ClassId extends u64 {}

/** @name ClassIdOf */
export interface ClassIdOf extends ClassId {}

/** @name ClassInfo */
export interface ClassInfo extends Struct {
  readonly metadata: Bytes;
  readonly total_issuance: TokenId;
  readonly owner: AccountId;
  readonly data: ClassData;
}

/** @name ClassInfoOf */
export interface ClassInfoOf extends ClassInfo {}

/** @name Currency */
export interface Currency extends AssetId {}

/** @name CurrencyId */
export interface CurrencyId extends AssetId {}

/** @name CurrencyIdOf */
export interface CurrencyIdOf extends AssetId {}

/** @name Fee */
export interface Fee extends Struct {
  readonly numerator: u32;
  readonly denominator: u32;
}

/** @name Intention */
export interface Intention extends Struct {
  readonly who: AccountId;
  readonly asset_sell: AssetId;
  readonly asset_buy: AssetId;
  readonly amount: Balance;
  readonly discount: bool;
  readonly sell_or_buy: IntentionType;
}

/** @name IntentionId */
export interface IntentionId extends Hash {}

/** @name IntentionType */
export interface IntentionType extends Enum {
  readonly isSell: boolean;
  readonly isBuy: boolean;
}

/** @name LBPWeight */
export interface LBPWeight extends u32 {}

/** @name LookupSource */
export interface LookupSource extends AccountId {}

/** @name MultiAsset */
export interface MultiAsset extends MultiAssetV1 {}

/** @name MultiLocation */
export interface MultiLocation extends MultiLocationV1 {}

/** @name OrderedSet */
export interface OrderedSet extends Vec<AssetId> {}

/** @name OrmlAccountData */
export interface OrmlAccountData extends Struct {
  readonly free: Balance;
  readonly frozen: Balance;
  readonly reserved: Balance;
}

/** @name Pool */
export interface Pool extends Struct {
  readonly owner: AccountId;
  readonly start: BlockNumber;
  readonly end: BlockNumber;
  readonly assets: AssetPair;
  readonly initial_weight: LBPWeight;
  readonly final_weight: LBPWeight;
  readonly weight_curve: WeightCurveType;
  readonly fee: Fee;
  readonly fee_collector: AccountId;
}

/** @name PoolId */
export interface PoolId extends AccountId {}

/** @name Price */
export interface Price extends Balance {}

/** @name TokenData */
export interface TokenData extends Struct {
  readonly locked: bool;
}

/** @name TokenId */
export interface TokenId extends u64 {}

/** @name TokenIdOf */
export interface TokenIdOf extends TokenId {}

/** @name TokenInfo */
export interface TokenInfo extends Struct {
  readonly metadata: Bytes;
  readonly owner: AccountId;
  readonly data: TokenData;
}

/** @name TokenInfoOf */
export interface TokenInfoOf extends TokenInfo {}

/** @name VestingSchedule */
export interface VestingSchedule extends Struct {
  readonly start: BlockNumber;
  readonly period: BlockNumber;
  readonly period_count: u32;
  readonly per_period: Compact<Balance>;
}

/** @name VestingScheduleOf */
export interface VestingScheduleOf extends VestingSchedule {}

/** @name WeightCurveType */
export interface WeightCurveType extends Enum {
  readonly isLinear: boolean;
}

/** @name Xcm */
export interface Xcm extends XcmV1 {}

/** @name XcmOrder */
export interface XcmOrder extends XcmOrderV1 {}

export type PHANTOM_BASILISK = 'basilisk';
