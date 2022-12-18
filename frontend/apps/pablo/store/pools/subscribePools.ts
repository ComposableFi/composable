import useStore from "@/store/useStore";
import { ApiPromise } from "@polkadot/api";
import { subscribePoolEntries } from "@/defi/utils";

export function subscribePools(api: ApiPromise) {
  return useStore.subscribe(
    (store) => ({
      isLoaded: store.substrateTokens.hasFetchedTokens,
      tokens: store.substrateTokens.tokens,
      setPoolConfig: store.pools.setConfig,
    }),
    ({ isLoaded, setPoolConfig, tokens }) => {
      let unsub = Promise.resolve(() => {});
      if (isLoaded) {
        unsub = subscribePoolEntries(api, tokens, setPoolConfig);
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
