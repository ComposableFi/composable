import { ApiPromise } from "@polkadot/api";
import { u128 } from "@polkadot/types-codec";

/**
 * Destination chain location
 * @param api Api
 * @param targetChain number
 * @returns XcmVersionedMultiLocation
 */
export const buildRelaychainToParachainDestination = (
  api: ApiPromise,
  targetChain: number
) =>
  api.createType("XcmVersionedMultiLocation", {
    V0: api.createType("XcmV0MultiLocation", {
      X1: api.createType("XcmV0Junction", {
        Parachain: api.createType("Compact<u32>", targetChain),
      }),
    }),
  });

/**
 * Required XCM Pallet args
 * Beneficiary of relay to para transfer
 * @param api Api
 * @param destinationAccount string
 * @returns XcmVersionedMultiLocation
 */
export const buildRelaychainToParachainBeneficiary = (
  api: ApiPromise,
  destinationAccount: string
) =>
  api.createType("XcmVersionedMultiLocation", {
    V0: api.createType("XcmV0MultiLocation", {
      X1: api.createType("XcmV0Junction", {
        AccountId32: {
          network: api.createType("XcmV0JunctionNetworkId", "Any"),
          id: api.createType("AccountId32", destinationAccount),
        },
      }),
    }),
  });

/**
 * Required XCM Pallet args
 * Transferrable Asset Param Via XCM
 * Only configured for KSM
 * @param api Api
 * @param amount u128
 * @returns XcmVersionedMultiAssets
 */
export const buildRelayChainXCMAsset = (api: ApiPromise, amount: u128) =>
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
