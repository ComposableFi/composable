// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { ComposableTraitsDefiCurrencyPairCurrencyId, CustomRpcBalance, CustomRpcCurrencyId, SafeRpcWrapper } from '@composable/types/interfaces/common';
import type { BTreeMap, Enum, Null, Struct, u128 } from '@polkadot/types-codec';
import type { AccountId32, AssetId, Balance, BlockNumber, Permill } from '@polkadot/types/interfaces/runtime';

/** @name ComposableTraitsDexFee */
export interface ComposableTraitsDexFee extends Struct {
  readonly fee: u128;
  readonly lp_fee: u128;
  readonly owner_fee: u128;
  readonly protocol_fee: u128;
  readonly asset_id: u128;
}

/** @name ComposableTraitsDexStakingRewardPool */
export interface ComposableTraitsDexStakingRewardPool extends Null {}

/** @name PalletPabloPoolConfiguration */
export interface PalletPabloPoolConfiguration extends Enum {
  readonly isDualAssetConstantProduct: boolean;
  readonly asDualAssetConstantProduct: {
    readonly owner: AccountId32;
    readonly assetsWeights: BTreeMap<AssetId, Permill>;
    readonly lpToken: u128;
    readonly feeConfig: {
    readonly feeRate: Permill;
    readonly ownerFeeRate: Permill;
    readonly protocolFeeRate: Permill;
  } & Struct;
  } & Struct;
  readonly isLiquidityBootstrapping: boolean;
  readonly asLiquidityBootstrapping: {
    readonly owner: AccountId32;
    readonly pair: ComposableTraitsDefiCurrencyPairCurrencyId;
    readonly sale: {
    readonly start: BlockNumber;
    readonly end: BlockNumber;
    readonly initial_weight: Permill;
    readonly final_weight: Permill;
  } & Struct;
  } & Struct;
  readonly type: 'DualAssetConstantProduct' | 'LiquidityBootstrapping';
}

/** @name PalletPabloPoolId */
export interface PalletPabloPoolId extends SafeRpcWrapper {}

/** @name PalletPabloPoolInitConfiguration */
export interface PalletPabloPoolInitConfiguration extends PalletPabloPoolConfiguration {}

/** @name PalletPabloPriceAggregate */
export interface PalletPabloPriceAggregate extends Struct {
  readonly poolId: PalletPabloPoolId;
  readonly baseAssetId: CustomRpcCurrencyId;
  readonly quoteAssetId: CustomRpcCurrencyId;
  readonly spotPrice: CustomRpcBalance;
}

/** @name PalletPabloPriceCumulative */
export interface PalletPabloPriceCumulative extends Null {}

/** @name PalletPabloTimeWeightedAveragePrice */
export interface PalletPabloTimeWeightedAveragePrice extends Null {}

/** @name RemoveLiquiditySimulationResult */
export interface RemoveLiquiditySimulationResult extends Struct {
  readonly assets: BTreeMap<AssetId, Balance>;
}

export type PHANTOM_PABLO = 'pablo';
