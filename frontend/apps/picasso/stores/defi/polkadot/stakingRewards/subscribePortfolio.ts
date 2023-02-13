import { useStore } from "@/stores/root";
import { ApiPromise } from "@polkadot/api";
import * as O from "fp-ts/lib/Option";
import { flow, pipe } from "fp-ts/function";
import * as A from "fp-ts/lib/ReadonlyArray";
import * as E from "fp-ts/lib/Either";
import {
  getFnftKey,
  tryFetchStakePortfolio,
} from "@/defi/polkadot/pallets/StakingRewards";
import config from "@/constants/config";

export function subscribePortfolio(api: ApiPromise | undefined) {
  return useStore.subscribe(
    (state) => ({
      stakingPositions: state.stakingPositions,
    }),
    async ({ stakingPositions }) => {
      if (!api) return;
      const rewardPools = useStore.getState().rewardPools;
      const picaAssetId = pipe(
        useStore.getState().substrateTokens.tokens.pica.chainId.picasso,
        O.fromNullable,
        O.map((s) => s.toString())
      );

      pipe(
        picaAssetId,
        O.map((assetId) =>
          pipe(
            Array.from(stakingPositions.entries()),
            A.map(([_, position]) =>
              tryFetchStakePortfolio(api, position, rewardPools, assetId)
            ),
            A.map((t) =>
              t().then(
                flow(
                  E.fold(
                    () => console.error("Could not fetch portfolio"),
                    (item) => {
                      useStore.setState((state) => {
                        state.stakingPortfolio.set(
                          getFnftKey(item.collectionId, item.instanceId),
                          item
                        );
                      });
                    }
                  )
                )
              )
            )
          )
        )
      );

      // TODO: Just for testing UI, remove before production
      if (config.stakingRewards.demoMode) {
        useStore.setState((state) => {
          state.stakingPortfolio = new Map(
            config.stakingRewards.picaPortfolios.map((item) => [
              getFnftKey(item.collectionId, item.instanceId),
              item,
            ])
          );
        });
      }
    },
    {
      equalityFn: (a, b) => a.stakingPositions === b.stakingPositions,
    }
  );
}
