import { useStore } from "@/stores/root";

export function subscribeTokenLoadedStatus() {
  return useStore.subscribe(
    (state) => state.substrateTokens.tokensLoaded,
    (loaded) => {
      useStore.setState((state) => {
        state.substrateTokens.isLoaded = Object.values(loaded).reduce(
          (acc, cur) => acc && cur,
          true
        );
      });
    }
  );
}
