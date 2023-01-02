import { ApiPromise } from "@polkadot/api";
import useStore from "@/store/useStore";
import { DEFAULT_NETWORK_ID, fromChainUnits } from "@/defi/utils";
import { Asset } from "shared";
import { getSubAccount } from "@/defi/utils/pablo/getSubAccount";

async function fetchInPool(
  api: ApiPromise,
  assetIn: Asset,
  assetOut: Asset,
  wallet: string
) {
  let inPoolAssetIn: any;
  let inPoolAssetOut: any;

  const assetInId = assetIn.getPicassoAssetId()?.toString() ?? "";
  const assetOutId = assetOut.getPicassoAssetId()?.toString() ?? "";
  if (assetInId === "1") {
    const out = await api.query.system.account(wallet);
    inPoolAssetIn = out.data;
  } else {
    inPoolAssetIn = await api.query.tokens.accounts(wallet, assetInId);
  }

  if (assetOutId === "1") {
    const out = await api.query.system.account(wallet);
    inPoolAssetOut = out.data;
  } else {
    inPoolAssetOut = await api.query.tokens.accounts(wallet, assetOutId);
  }

  return {
    [assetInId]: fromChainUnits(
      inPoolAssetIn.free.toString(),
      assetIn.getDecimals(DEFAULT_NETWORK_ID)
    ).toString(),
    [assetOutId]: fromChainUnits(
      inPoolAssetOut.free.toString(),
      assetOut.getDecimals(DEFAULT_NETWORK_ID)
    ).toString(),
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
        const assetIn = pool.config.assets[0];
        const assetOut = pool.config.assets[1];
        const ownerWalletAddress = getSubAccount(api, pool.poolId.toString());

        const amount = await fetchInPool(
          api,
          assetIn,
          assetOut,
          ownerWalletAddress
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
