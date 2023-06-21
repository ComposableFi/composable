import useStore from "@/store/useStore";
import { ApiPromise } from "@polkadot/api";
import { subscribePoolEntries } from "@/defi/utils";

export function subscribePools(api: ApiPromise) {
  return useStore.subscribe(
    (store) => ({
      isLoaded: store.substrateTokens.hasFetchedTokens,
      isPoolsLoaded: store.pools.isLoaded,
    }),
    ({ isLoaded, isPoolsLoaded }) => {
      let unsub = Promise.resolve(() => {});
      if (isLoaded && !isPoolsLoaded) {
        console.log("[Fetch pools]: initializing");
        unsub = subscribePoolEntries(
          api,
          useStore.getState().substrateTokens.tokens,
          useStore.getState().pools.setConfig
        );
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
