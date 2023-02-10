import { useStore } from "@/stores/root";
import { ApiPromise } from "@polkadot/api";
import * as O from "fp-ts/lib/Option";
import { flow, pipe } from "fp-ts/function";
import * as A from "fp-ts/lib/ReadonlyArray";
import * as E from "fp-ts/lib/Either";
import { tryFetchStakePortfolio } from "@/defi/polkadot/pallets/StakingRewards";
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

      useStore.setState((state) => {
        state.stakingPortfolio = [];
      });

      pipe(
        picaAssetId,
        O.map((assetId) =>
          pipe(
            stakingPositions,
            A.map((position) =>
              tryFetchStakePortfolio(api, position, rewardPools, assetId)
            ),
            A.map((t) =>
              t().then(
                flow(
                  E.fold(
                    () => console.error("Could not fetch portfolio"),
                    (item) => {
                      useStore.setState((state) => {
                        state.stakingPortfolio = [
                          ...state.stakingPortfolio,
                          item,
                        ];
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
          state.stakingPortfolio = config.stakingRewards.picaPortfolios;
        });
      }
    },
    {
      equalityFn: (a, b) => a.stakingPositions === b.stakingPositions,
    }
  );
}
