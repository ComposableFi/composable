// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type {
  ComposableTraitsDefiCurrencyPairCurrencyId,
  CustomRpcBalance,
  CustomRpcCurrencyId,
  SafeRpcWrapper
} from "@composable/common";
import type { Enum, Null, Struct, u128, u16 } from "@polkadot/types-codec";
import type { AccountId32, BlockNumber, Permill } from "@polkadot/types/interfaces/runtime";

/** @name ComposableTraitsDexFee */
export interface ComposableTraitsDexFee extends Struct {
  readonly fee: u128;
  readonly lp_fee: u128;
  readonly owner_fee: u128;
  readonly protocol_fee: u128;
  readonly asset_id: u128;
}

/** @name PalletPabloPoolConfiguration */
export interface PalletPabloPoolConfiguration extends Enum {
  readonly isStableSwap: boolean;
  readonly asStableSwap: {
    readonly owner: AccountId32;
    readonly pair: ComposableTraitsDefiCurrencyPairCurrencyId;
    readonly amplification_coefficient: u16;
    readonly fee: Permill;
    readonly ownerFee: Permill;
  } & Struct;
  readonly isConstantProduct: boolean;
  readonly asConstantProduct: {
    readonly owner: AccountId32;
    readonly pair: ComposableTraitsDefiCurrencyPairCurrencyId;
    readonly fee: Permill;
    readonly ownerFee: Permill;
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
    readonly fee: Permill;
  } & Struct;
  readonly type: "StableSwap" | "ConstantProduct" | "LiquidityBootstrapping";
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

export type PHANTOM_PABLO = "pablo";
