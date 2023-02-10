import { ApiPromise } from "@polkadot/api";
import { useStore } from "@/stores/root";
import { flow, pipe } from "fp-ts/function";
import * as O from "fp-ts/lib/Option";
import * as E from "fp-ts/lib/Either";
import { tryFetchStakes } from "@/defi/polkadot/pallets/StakingRewards";
import { fromChainIdUnit } from "shared";
import BigNumber from "bignumber.js";
import { StorageKey } from "@polkadot/types";
import { Option, u128, u64 } from "@polkadot/types-codec";
import { ComposableTraitsStakingStake } from "defi-interfaces";

function sumAllStakes(
  entries: [StorageKey<[u128, u64]>, Option<ComposableTraitsStakingStake>][]
) {
  return entries
    .map(([_, optionEntry]) =>
      fromChainIdUnit(optionEntry.unwrap().stake.toString())
    )
    .reduce((acc, cur) => acc.plus(cur), new BigNumber(0));
}

function sumAllShares(
  entries: [StorageKey<[u128, u64]>, Option<ComposableTraitsStakingStake>][]
) {
  return entries
    .map(([_, optionEntry]) =>
      fromChainIdUnit(optionEntry.unwrap().share.toString())
    )
    .reduce((acc, cur) => acc.plus(cur), new BigNumber(0));
}

export function subscribeMaximumPICAStaked(api: ApiPromise | undefined) {
  return useStore.subscribe(
    (state) => state.substrateTokens.isLoaded,
    (isLoaded) => {
      if (!isLoaded) return;

      const getStakesCall = pipe(
        O.fromNullable(api),
        O.map((a) => tryFetchStakes(a, "2001"))
      );

      pipe(
        getStakesCall,
        O.map((call) =>
          call().then(
            flow(
              E.bindTo("stakes"),
              E.bind("maxStakes", ({ stakes }) => E.of(sumAllStakes(stakes))),
              E.bind("maxShares", ({ stakes }) => E.of(sumAllShares(stakes))),
              E.map(({ maxShares, maxStakes }) => {
                useStore.getState().setMaxPICAStakes(maxStakes);
                useStore.getState().setMaxPICAShares(maxShares);
              })
            )
          )
        )
      );
    },
    {
      fireImmediately: true,
      equalityFn: (a, b) => b && a === b,
    }
  );
}
