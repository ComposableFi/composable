import { ApiPromise } from "@polkadot/api";
import { useStore } from "@/stores/root";
import { subscribePoolEntries } from "@/stores/defi/polkadot/pablo/subscribePoolEntries";

export function subscribePools(api: ApiPromise) {
  return useStore.subscribe(
    (store) => ({
      isLoaded: store.substrateTokens.isLoaded,
      isPoolsLoaded: store.pools.isLoaded,
    }),
    ({ isLoaded, isPoolsLoaded }) => {
      console.log("[Pool subscription]: start", isLoaded, isPoolsLoaded);
      let unsub = Promise.resolve(() => {});
      if (isLoaded && !isPoolsLoaded) {
        console.log("[Pool subscription]: initialize");
        unsub = subscribePoolEntries(
          api,
          useStore.getState().substrateTokens.tokens,
          useStore.getState().pools.setConfig
        );
        console.log("[Pool subscription]: done");
      }

      return () => {
        unsub.then((fn) => fn());
      };
    },
    {
      fireImmediately: true,
      equalityFn: (a, b) => b.isLoaded && b.isPoolsLoaded,
    }
  );
}
