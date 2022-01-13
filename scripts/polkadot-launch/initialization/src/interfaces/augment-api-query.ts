// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api/types';
import type { BTreeMap, Bytes, Null, Option, Vec, bool, u128, u16, u32, u64 } from '@polkadot/types';
import type { AccountId32, Call, H256 } from '@polkadot/types/interfaces/runtime';
import type { AnyNumber, ITuple, Observable } from '@polkadot/types/types';
import type { AssetInstanceV1, MultiAssetV1, MultiLocationV1 as XcmV1MultiLocation, XcmOrderV1, XcmV1 } from '@polkadot/types/interfaces/xcm';
import type { 
    AssetDetails as PalletAssetRegistryAssetDetails,
    AssetMetadata as PalletAssetRegistryAssetMetadata,
    AssetNativeLocation as TestingBasiliskRuntimeAssetLocation
} from './basilisk';

declare module '@polkadot/api/types/storage' {
  export interface AugmentedQueries<ApiType> {
    assetRegistry: {
      /**
       * Mapping between asset name and asset id.
       **/
      assetIds: AugmentedQuery<ApiType, (arg: Bytes | string | Uint8Array) => Observable<Option<u32>>, [Bytes]> & QueryableStorageEntry<ApiType, [Bytes]>;
      /**
       * Native location of an asset.
       **/
      assetLocations: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<XcmV1MultiLocation>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Metadata of an asset.
       **/
      assetMetadataMap: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletAssetRegistryAssetMetadata>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Details of an asset.
       **/
      assets: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletAssetRegistryAssetDetails>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Local asset for native location.
       **/
      locationAssets: AugmentedQuery<ApiType, (arg: TestingBasiliskRuntimeAssetLocation | { parents?: any; interior?: any } | string | Uint8Array) => Observable<Option<u32>>, [TestingBasiliskRuntimeAssetLocation]> & QueryableStorageEntry<ApiType, [TestingBasiliskRuntimeAssetLocation]>;
      /**
       * Next available asset id. This is sequential id assigned for each new registered asset.
       **/
      nextAssetId: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
  }

  export interface QueryableStorage<ApiType extends ApiTypes> extends AugmentedQueries<ApiType> {
    [key: string]: QueryableModuleStorage<ApiType>;
  }
}
