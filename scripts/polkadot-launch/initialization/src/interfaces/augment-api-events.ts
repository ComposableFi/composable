// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api/types';
import type { Bytes, Null, Option, Result, U8aFixed, Vec, bool, i128, u128, u32, u64, u8 } from '@polkadot/types';
import type { AccountId32, H256 } from '@polkadot/types/interfaces/runtime';
import type { ITuple } from '@polkadot/types/types';
import type { AssetNativeLocation as TestingBasiliskRuntimeAssetLocation, AssetType as PalletAssetRegistryAssetType } from './basilisk';

declare module '@polkadot/api/types/events' {
  export interface AugmentedEvents<ApiType> {
    assetRegistry: {
      /**
       * Native location set for an asset. \[asset_id, location\]
       **/
      LocationSet: AugmentedEvent<ApiType, [u32, TestingBasiliskRuntimeAssetLocation]>;
      /**
       * Metadata set for an asset. \[asset_id, symbol, decimals\]
       **/
      MetadataSet: AugmentedEvent<ApiType, [u32, Bytes, u8]>;
      /**
       * Asset was registered. \[asset_id, name, type\]
       **/
      Registered: AugmentedEvent<ApiType, [u32, Bytes, PalletAssetRegistryAssetType]>;
      /**
       * Asset was updated. \[asset_id, name, type\]
       **/
      Updated: AugmentedEvent<ApiType, [u32, Bytes, PalletAssetRegistryAssetType]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
  }

  export interface DecoratedEvents<ApiType extends ApiTypes> extends AugmentedEvents<ApiType> {
    [key: string]: ModuleEvents<ApiType>;
  }
}
