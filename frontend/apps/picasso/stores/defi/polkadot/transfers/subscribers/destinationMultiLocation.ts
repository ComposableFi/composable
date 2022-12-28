import { AllProviders } from "@/defi/polkadot/context/hooks";
import { useStore } from "@/stores/root";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { XcmVersionedMultiLocation } from "@polkadot/types/lookup";

export const subscribeDestinationMultiLocation = async (
  allProviders: AllProviders,
  targetAddress: string
) => {
  return useStore.subscribe(
    (state) => ({
      targetChain: state.transfers.networks.to,
      sourceChain: state.transfers.networks.from,
      selectedAddress: state.transfers.recipients.selected,
      amount: state.transfers.amount,
    }),
    ({ sourceChain, targetChain, selectedAddress }) => {
      const api = allProviders[sourceChain]?.parachainApi;
      if (!api) return;

      const targetChainId = SUBSTRATE_NETWORKS[targetChain].parachainId;
      const recipient = selectedAddress.length
        ? selectedAddress
        : targetAddress;
      const set = useStore.getState().transfers.setDestinationMultiLocation;

      // Set to null to re-trigger all listeners
      set(null);
      // Kusama to Picasso uses XCM standard address
      if (sourceChain === "kusama") {
        set(
          api.createType("XcmVersionedMultiLocation", {
            V0: api.createType("XcmV0MultiLocation", {
              X1: api.createType("XcmV0Junction", {
                Parachain: api.createType("u32", targetChainId),
              }),
            }),
          }) as XcmVersionedMultiLocation
        );
      }

      if (sourceChain === "statemine") {
        set(
          <XcmVersionedMultiLocation>api.createType(
            "XcmVersionedMultiLocation",
            {
              V1: {
                parents: 1,
                interior: {
                  X1: {
                    Parachain: targetChainId,
                  },
                },
              },
            }
          )
        );
      }

      // Picasso to Kusama needs recipient in MultiLocation
      if (
        sourceChain === "picasso" &&
        ["kusama"].includes(targetChain) &&
        recipient
      ) {
        // Set destination. Should have 2 Junctions, first to parent and then to wallet
        set(
          <XcmVersionedMultiLocation>api.createType(
            "XcmVersionedMultiLocation",
            {
              V0: api.createType("XcmV0MultiLocation", {
                X2: [
                  api.createType("XcmV0Junction", "Parent"),
                  api.createType("XcmV0Junction", {
                    AccountId32: {
                      network: api.createType("XcmV0JunctionNetworkId", "Any"),
                      id: api.createType("AccountId32", recipient),
                    },
                  }),
                ],
              }),
            }
          )
        );
      } else if (
        sourceChain === "picasso" &&
        targetChain === "statemine" &&
        recipient
      ) {
        const dest = api.createType("XcmVersionedMultiLocation", {
          V1: api.createType("XcmV1MultiLocation", {
            parents: api.createType("u8", 1),
            interior: api.createType("XcmV1MultilocationJunctions", {
              X2: [
                api.createType("XcmV1Junction", {
                  Parachain: api.createType("Compact<u32>", 1000),
                }),
                api.createType("XcmV1Junction", {
                  AccountId32: {
                    network: api.createType("XcmV0JunctionNetworkId", "Any"),
                    id: api.createType("AccountId32", recipient),
                  },
                }),
              ],
            }),
          }),
        });
        set(dest as any);
      }

      // Karura <> Picasso needs recipient in MultiDestLocation
      if ([sourceChain, targetChain].includes("karura") && recipient) {
        set(
          api.createType("XcmVersionedMultiLocation", {
            V0: api.createType("XcmV0MultiLocation", {
              X3: [
                api.createType("XcmV0Junction", "Parent"),
                api.createType("XcmV0Junction", {
                  Parachain: api.createType("Compact<u32>", targetChainId),
                }),
                api.createType("XcmV0Junction", {
                  AccountId32: {
                    network: api.createType("XcmV0JunctionNetworkId", "Any"),
                    id: api.createType("AccountId32", recipient),
                  },
                }),
              ],
            }),
          }) as XcmVersionedMultiLocation
        );
      }
    },
    {
      fireImmediately: true,
      equalityFn: (
        { sourceChain, targetChain, selectedAddress },
        {
          sourceChain: $sourceChain,
          targetChain: $targetChain,
          selectedAddress: $selectedAddress,
        }
      ) => {
        return (
          sourceChain === $sourceChain &&
          targetChain === $targetChain &&
          selectedAddress === $selectedAddress
        );
      },
    }
  );
};
