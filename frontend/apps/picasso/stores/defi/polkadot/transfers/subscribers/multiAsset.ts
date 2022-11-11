import { AllProviders } from "@/defi/polkadot/context/hooks";
import { useStore } from "@/stores/root";
import {
  AcalaPrimitivesCurrencyCurrencyId,
  XcmVersionedMultiAsset,
  XcmVersionedMultiAssets,
} from "@acala-network/types/interfaces/types-lookup";

export const subscribeMultiAsset = async (allProviders: AllProviders) => {
  return useStore.subscribe(
    (store) => ({
      selectedToken: store.transfers.selectedToken,
      from: store.transfers.networks.from,
      to: store.transfers.networks.to,
      amount: store.transfers.amount,
      keepAlive: store.transfers.keepAlive,
      existentialDeposit: store.transfers.existentialDeposit,
    }),
    ({ selectedToken, from, to, amount }) => {
      const api = allProviders[from].parachainApi;
      if (!api) return;
      const amountToTransfer = useStore
        .getState()
        .transfers.getTransferAmount(api);
      const selectedTokenId =
        useStore.getState().substrateTokens.tokens[selectedToken].chainId
          .picasso;
      const set = useStore.getState().transfers.setTransferMultiAsset;
      // Set this to null to re-trigger all listeners
      set(null);
      if (selectedTokenId === null) return;

      if (from === "kusama" && to === "picasso") {
        set(
          api.createType("XcmVersionedMultiAssets", {
            V0: [
              api.createType("XcmV0MultiAsset", {
                ConcreteFungible: {
                  id: api.createType("XcmV0MultiLocation", "Null"),
                  amount: amountToTransfer,
                },
              }),
            ],
          }) as XcmVersionedMultiAsset
        );
      }

      if (from === "statemine") {
        if (selectedToken === "usdt") {
          set(
            <XcmVersionedMultiAssets>api.createType("XcmVersionedMultiAssets", {
              V1: [
                {
                  id: {
                    Concrete: {
                      parents: 0,
                      interior: {
                        X2: [
                          {
                            PalletInstance: 50,
                          },
                          {
                            GeneralIndex:
                              useStore.getState().substrateTokens.tokens[
                                selectedToken
                              ].chainId.statemine,
                          },
                        ],
                      },
                    },
                  },
                  fun: {
                    Fungible: amountToTransfer.toString(),
                  },
                },
              ],
            })
          );
        }
        if (selectedToken === "ksm") {
          set(
            <XcmVersionedMultiAssets>api.createType("XcmVersionedMultiAssets", {
              V1: [
                {
                  id: {
                    Concrete: {
                      parents: 1,
                      interior: "Here",
                    },
                  },
                  fun: {
                    Fungible: amountToTransfer.toString(),
                  },
                },
              ],
            })
          );
        }
      }

      if (from === "karura" && to === "picasso") {
        set(
          api.createType("AcalaPrimitivesCurrencyCurrencyId", {
            Token: api.createType(
              "AcalaPrimitivesCurrencyTokenSymbol",
              selectedToken.toUpperCase()
            ),
          }) as AcalaPrimitivesCurrencyCurrencyId
        );
      }

      if (from === "picasso") {
        set(api.createType("u128", selectedTokenId.toString()));
      }
    },
    {
      fireImmediately: true,
      equalityFn: (a, b) =>
        a.selectedToken === b.selectedToken &&
        a.from === b.from &&
        a.to === b.to &&
        a.amount === b.amount,
    }
  );
};
