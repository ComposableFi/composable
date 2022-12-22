import useStore from "../useStore";
import { pipe } from "fp-ts/lib/function";
import { readonlyArray } from "fp-ts";
import { ApiPromise } from "@polkadot/api";
import { Asset, subscribePicassoBalanceByAssetId } from "shared";
import BigNumber from "bignumber.js";
import { PoolConfig } from "@/store/pools/types";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";

function getTokenPair(config: PoolConfig) {
  const [a, b] = config.config.assets;

  return [a, b] as [Asset, Asset];
}

let prevSub: any = undefined;

export function subscribeOwnedLiquidity(
  api: ApiPromise,
  accountAddress: string
) {
  return useStore.subscribe(
    (store) => ({
      isPoolsLoaded: store.pools.isLoaded,
    }),
    ({ isPoolsLoaded }) => {
      if (typeof prevSub === "function") {
        prevSub?.();
      }
      if (!isPoolsLoaded) return;
      const tokens = useStore.getState().substrateTokens.tokens;
      const config = useStore.getState().pools.config;
      const setOwnedLiquidityToken =
        useStore.getState().ownedLiquidity.setOwnedLiquidity;
      // Fetch LP Tokens from config
      pipe(
        readonlyArray.fromArray(config),
        readonlyArray.map((configItem) => {
          const asset = Object.values(tokens).find(
            (token) =>
              token.getPicassoAssetId()?.toString() ===
              configItem.config.lpToken.toString()
          );
          prevSub = subscribePicassoBalanceByAssetId(
            api,
            accountAddress,
            new BigNumber(configItem.config.lpToken),
            asset?.getDecimals(DEFAULT_NETWORK_ID) || 12,
            (balanceData) => {
              const pair = getTokenPair(configItem);
              const poolId = configItem.poolId;
              setOwnedLiquidityToken(
                configItem.config.lpToken,
                balanceData,
                pair,
                poolId
              );
            }
          );
        })
      );
    },
    {
      fireImmediately: true,
      equalityFn: (a, b) => a.isPoolsLoaded && b.isPoolsLoaded,
    }
  );
}
