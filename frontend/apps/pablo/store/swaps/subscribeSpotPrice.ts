import useStore from "@/store/useStore";
import { ApiPromise } from "@polkadot/api";
import { fetchSpotPrice } from "@/defi/utils";

export function subscribeSpotPrice(api: ApiPromise) {
  return useStore.subscribe(
    (store) => ({
      pool: store.swaps.selectedPool,
      assetOne: store.swaps.selectedAssets.base,
      assetTwo: store.swaps.selectedAssets.quote,
    }),
    async ({ pool, assetOne, assetTwo }) => {
      if (!pool || assetOne === "none" || assetTwo === "none") {
        return;
      }

      console.log("Subscribing to spotPrice");

      const setSpotPrice = useStore.getState().swaps.setSpotPrice;

      setSpotPrice(await fetchSpotPrice(api, pool, assetOne, assetTwo));
    },
    {
      fireImmediately: true,
      equalityFn: (a, b) =>
        a.assetTwo === b.assetTwo &&
        a.assetOne === b.assetOne &&
        !!b.pool &&
        a.pool?.poolId.toString() === b.pool?.poolId.toString(),
    }
  );
}
