import useStore from "@/store/useStore";
import { ApiPromise } from "@polkadot/api";
import { subscribePoolEntries } from "@/defi/utils";

export function subscribePools(api: ApiPromise) {
  return useStore.subscribe(
    (store) => ({
      isLoaded: store.substrateTokens.hasFetchedTokens,
      setPoolConfig: store.pools.setConfig,
    }),
    ({ isLoaded, setPoolConfig }) => {
      let unsub = Promise.resolve(() => {});
      if (isLoaded) {
        unsub = subscribePoolEntries(api, setPoolConfig);
      }

      return () => {
        unsub.then((fn) => fn());
      };
    },
    {
      equalityFn: (a, b) => a.isLoaded === b.isLoaded,
    }
  );
}
