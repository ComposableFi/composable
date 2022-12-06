import { ApiPromise } from "@polkadot/api";
import { fromChainIdUnit } from "shared";
import BigNumber from "bignumber.js";

import * as TaskEither from "fp-ts/TaskEither";
import { flow, pipe } from "fp-ts/function";
import * as Option from "fp-ts/lib/Option";
import useStore from "@/store/useStore";

function toString(value: unknown): string {
  if (value instanceof Number || value instanceof String) {
    return value.toString();
  }

  return "";
}

function strToBN(value: string): BigNumber {
  return fromChainIdUnit(new BigNumber(value));
}

function anyToBigNumber(value: unknown) {
  return pipe(value, toString, strToBN);
}

function fetchEd(api: ApiPromise) {
  return function (a: string | BigNumber) {
    return TaskEither.tryCatch(
      async () => await api.query.currencyFactory.assetEd(a.toString()),
      (reason) => new Error(String(reason))
    );
  };
}

export const subscribeFeeItemEd = async (api: ApiPromise) => {
  return useStore.subscribe(
    (store) => ({
      feeItem: store.byog.feeItem,
      isLoaded: store.substrateTokens.hasFetchedTokens,
    }),
    async ({ feeItem, isLoaded }) => {
      if (!isLoaded) return;

      pipe(
        Option.fromNullable(
          useStore
            .getState()
            .substrateTokens.tokens[feeItem].getIdOnChain("picasso")
        ),
        Option.map(fetchEd(api)),
        Option.map(
          flow(
            TaskEither.map(anyToBigNumber),
            TaskEither.map((existentialValue) => {
              useStore
                .getState()
                .byog.setFeeItemEd(
                  existentialValue.isNaN() ? new BigNumber(0) : existentialValue
                );
            })
          )
        )
      );
    },
    {
      fireImmediately: true,
      equalityFn: (a, b) =>
        a.feeItem === b.feeItem && a.isLoaded === b.isLoaded,
    }
  );
};
