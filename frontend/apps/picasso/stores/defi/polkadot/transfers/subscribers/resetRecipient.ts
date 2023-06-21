import { useStore } from "@/stores/root";

export function resetRecipient() {
  return useStore.subscribe(
    (state) => state.transfers.networks.to,
    (to) => useStore.getState().transfers.updateRecipient("")
  );
}
