import { AllProviders } from "@/defi/polkadot/context/hooks";
import { useStore } from "@/stores/root";

export const subscribeTransferApiCall = async (allProviders: AllProviders) => {
  return useStore.subscribe(
    (store) => ({
      from: store.transfers.networks.from,
      to: store.transfers.networks.to,
      selectedToken: store.transfers.selectedToken,
      amount: store.transfers.amount,
    }),
    ({ from, to }) => {
      const api = allProviders[from].parachainApi;
      if (!api) return;
      const set = useStore.getState().transfers.setTransferExtrinsic;
      // Set to null to re-trigger all listeners
      set(null);
      if (from === "kusama" && to === "picasso") {
        try {
          set(api.tx.xcmPallet.reserveTransferAssets);
        } catch (e) {
          console.log("could not create API: xcmPallet not ready");
        }
      }

      if (from === "karura" && to === "picasso") {
        set(api.tx.xTokens.transfer);
      }

      if (from === "statemine") {
        set(api.tx.polkadotXcm.limitedReserveTransferAssets);
      }

      // Both Karura and Kusama as targetChain
      if (from === "picasso") {
        try {
          set(api.tx.xTokens.transfer);
        } catch (e) {
          console.log("Could not create API: xTokens not ready.", e);
        }
      }
    },
    {
      fireImmediately: true,
      equalityFn: (a, b) => a.from === b.from && a.to === b.to,
    }
  );
};
