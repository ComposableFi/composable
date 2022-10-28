import { ApiPromise } from "@polkadot/api";
/**
 * Build parachain XCM Destination arg given chain id
 * and account from a parachain
 * @param api Origin Chain API
 * @param targetChain numeric u32 parachain id
 * @param targetAccount account id (could be ethereum account?)
 * @returns {XcmVersionedMultiLocation}
 */
export const buildParachainToParachainAccountDestination = (
  api: ApiPromise,
  targetChain: number,
  targetAccount: string
) =>
  api.createType("XcmVersionedMultiLocation", {
    V0: api.createType("XcmV0MultiLocation", {
      X3: [
        api.createType("XcmV0Junction", "Parent"),
        api.createType("XcmV0Junction", {
          Parachain: api.createType("Compact<u32>", targetChain),
        }),
        api.createType("XcmV0Junction", {
          AccountId32: {
            network: api.createType("XcmV0JunctionNetworkId", "Any"),
            id: api.createType("AccountId32", targetAccount),
          },
        }),
      ],
    }),
  });
/**
 * Build Relaychain XCM Destination arg given chain id
 * and account from a parachain
 * @param api Origin Chain API
 * @param targetAccount account id (could be ethereum account?)
 * @returns {XcmVersionedMultiLocation}
 */
export const buildParachainToRelaychainAccountDestination = (
  api: ApiPromise,
  targetAccount: string
) =>
  api.createType("XcmVersionedMultiLocation", {
    V0: api.createType("XcmV0MultiLocation", {
      X2: [
        api.createType("XcmV0Junction", "Parent"),
        api.createType("XcmV0Junction", {
          AccountId32: {
            network: api.createType("XcmV0JunctionNetworkId", "Any"),
            id: api.createType("AccountId32", targetAccount),
          },
        }),
      ],
    }),
  });
