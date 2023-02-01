import { useStore } from "@/stores/root";
import { ApiPromise } from "@polkadot/api";
import * as O from "fp-ts/lib/Option";
import { flow, pipe } from "fp-ts/function";
import * as A from "fp-ts/lib/ReadonlyArray";
import * as E from "fp-ts/lib/Either";
import { tryFetchStakePortfolio } from "@/defi/polkadot/pallets/StakingRewards";

export function subscribePortfolio(api: ApiPromise | undefined) {
  return useStore.subscribe(
    (state) => ({
      isStakingPositionsLoading: state.isStakingPositionsLoading,
    }),
    async ({ isStakingPositionsLoading }) => {
      if (!api || isStakingPositionsLoading) return;

      const stakingPositions = useStore.getState().stakingPositions;
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
    }
  );
}
