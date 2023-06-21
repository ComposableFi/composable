import { ApiPromise } from "@polkadot/api";
import { fromChainIdUnit } from "shared";
import BigNumber from "bignumber.js";

import * as TaskEither from "fp-ts/TaskEither";
import { pipe } from "fp-ts/function";
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

function anyToBigNumber(value: any) {
  return pipe(value.isSome ? value.toString() : "0", strToBN);
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

      const ed = useStore
        .getState()
        .substrateTokens.tokens[feeItem].getExistentialDeposit("picasso");

      if (ed) {
        useStore.getState().byog.setFeeItemEd(ed);
        useStore.getState().byog.setLoaded(true);
      }
    },
    {
      fireImmediately: true,
      equalityFn: (a, b) =>
        a.feeItem === b.feeItem && a.isLoaded === b.isLoaded,
    }
  );
};
