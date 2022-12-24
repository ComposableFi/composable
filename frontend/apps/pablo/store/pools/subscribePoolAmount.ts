import { ApiPromise } from "@polkadot/api";
import useStore from "@/store/useStore";
import { fromChainUnits } from "@/defi/utils";

async function fetchInPool(
  api: ApiPromise,
  assetIn: string,
  assetOut: string,
  wallet: string
) {
  const inPoolAssetIn = await api.query.tokens.accounts(wallet, assetIn);
  const inPoolAssetOut = await api.query.tokens.accounts(wallet, assetOut);

  return {
    [assetIn]: fromChainUnits(inPoolAssetIn.free.toString()).toString(),
    [assetOut]: fromChainUnits(inPoolAssetOut.free.toString()).toString(),
  };
}

export function subscribePoolAmount(api: ApiPromise | undefined) {
  return useStore.subscribe(
    (store) => ({
      isPoolLoaded: store.pools.isLoaded,
    }),
    async ({ isPoolLoaded }) => {
      if (!api || !isPoolLoaded) return;

      const setPoolAmount = useStore.getState().pools.setPoolAmount;
      const setTotalIssued = useStore.getState().pools.setTotalIssued;

      const pools = useStore.getState().pools.config;
      for (const pool of pools) {
        const assetIn = pool.config.assets[0].getPicassoAssetId() as string;
        const assetOut = pool.config.assets[1].getPicassoAssetId() as string;
        const amount = await fetchInPool(
          api,
          assetIn,
          assetOut,
          pool.config.owner
        );

        setPoolAmount(pool.poolId.toString(), amount);
        api.query.tokens
          .totalIssuance(pool.config.lpToken.toString())
          .then((total) => {
            setTotalIssued(pool.poolId, fromChainUnits(total.toString(), 12));
          });
      }
    },
    {
      equalityFn: (a, b) => a.isPoolLoaded && b.isPoolLoaded,
      fireImmediately: true,
    }
  );
}
