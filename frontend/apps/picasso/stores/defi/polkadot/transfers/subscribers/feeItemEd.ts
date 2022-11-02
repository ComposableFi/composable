import { ApiPromise } from "@polkadot/api";
import { useStore } from "@/stores/root";
import { fromChainIdUnit } from "shared";
import BigNumber from "bignumber.js";

import * as TaskEither from "fp-ts/TaskEither";
import { flow, pipe } from "fp-ts/function";
import * as Option from "fp-ts/lib/Option";

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

export const subscribeFeeItemEd = async (api: ApiPromise) => {
  return useStore.subscribe(
    (store) => ({
      feeItem: store.transfers.feeItem,
      sourceChain: store.transfers.networks.from,
      isLoaded: store.substrateTokens.isLoaded,
    }),
    async ({ feeItem, isLoaded, sourceChain }) => {
      if (!isLoaded) return;

      function fetchEd(api: ApiPromise) {
        return function (a: string | BigNumber) {
          return TaskEither.tryCatch(
            async () => await api.query.currencyFactory.assetEd(a.toString()),
            (reason) => new Error(String(reason))
          );
        };
      }

      pipe(
        Option.fromNullable(
          useStore.getState().substrateTokens.tokens[feeItem].chainId[
            sourceChain
          ]
        ),
        Option.map(fetchEd(api)),
        Option.map(
          flow(
            TaskEither.map(anyToBigNumber),
            TaskEither.map((existentialValue) => {
              useStore.setState({
                ...useStore.getState(),
                transfers: {
                  ...useStore.getState().transfers,
                  feeItemEd: existentialValue.isNaN()
                    ? new BigNumber(0)
                    : existentialValue,
                },
              });
            })
          )
        )
      );
    },
    {
      fireImmediately: true,
      equalityFn: (a, b) =>
        a.feeItem === b.feeItem &&
        a.sourceChain === b.sourceChain &&
        a.isLoaded === b.isLoaded,
    }
  );
};
