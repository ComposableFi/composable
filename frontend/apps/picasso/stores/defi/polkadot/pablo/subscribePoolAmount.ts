import { ApiPromise } from "@polkadot/api";
import { fromChainIdUnit, getSubAccount } from "shared";
import { useStore } from "@/stores/root";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";

async function fetchInPool(
  api: ApiPromise,
  assetIn: TokenMetadata,
  assetOut: TokenMetadata,
  wallet: string
) {
  let inPoolAssetIn: any;
  let inPoolAssetOut: any;

  const assetInId = assetIn.chainId.picasso?.toString() ?? "";
  const assetOutId = assetOut.chainId.picasso?.toString() ?? "";
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
    [assetInId]: fromChainIdUnit(
      inPoolAssetIn.free.toString(),
      assetIn.decimals.picasso ?? 12
    ).toString(),
    [assetOutId]: fromChainIdUnit(
      inPoolAssetOut.free.toString(),
      assetOut.decimals.picasso ?? 12
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

        console.log(ownerWalletAddress);

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
            setTotalIssued(pool.poolId, fromChainIdUnit(total.toString(), 12));
          });
      }
    },
    {
      equalityFn: (a, b) => a.isPoolLoaded && b.isPoolLoaded,
      fireImmediately: true,
    }
  );
}
