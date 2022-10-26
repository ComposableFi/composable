import { ApiPromise } from "@polkadot/api";

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

