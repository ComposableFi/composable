import { ApiPromise } from "@polkadot/api";
import { u128 } from "@polkadot/types";

/**
 * Transferrable Asset Param Via XCM
 * Only configured for KSM
 * @param api Api
 * @param amount u128
 * @returns XcmVersionedMultiAssets
 */
 export const buildXCMAssetOriginKsm = (api: ApiPromise, amount: u128) =>
 api.createType("XcmVersionedMultiAssets", {
   V0: [
     api.createType("XcmV0MultiAsset", {
       ConcreteFungible: {
         id: api.createType("XcmV0MultiLocation", "Null"),
         amount,
       },
     }),
   ],
 });
