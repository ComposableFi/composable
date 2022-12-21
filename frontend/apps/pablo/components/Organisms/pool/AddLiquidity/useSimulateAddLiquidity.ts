import { useParachainApi, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID, fromChainUnits } from "@/defi/utils";
import { useCallback } from "react";
import { pipe } from "fp-ts/lib/function";
import { either, option, taskEither } from "fp-ts";
import { tryCatch } from "fp-ts/TaskEither";
import BigNumber from "bignumber.js";
import { ApiPromise } from "@polkadot/api";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { getAssetTree } from "@/components/Organisms/pool/AddLiquidity/utils";

export type AssetWithBalance = {
  assetIdOnChain: string;
  balance: BigNumber;
};

function trySimulateAddLiquidity(
  api: ApiPromise,
  account: InjectedAccountWithMeta,
  poolId: string,
  assetTree: { [x: string]: string }
) {
  // @ts-ignore
  return pipe(
    tryCatch(
      () =>
        api.rpc.pablo.simulateAddLiquidity(account.address, poolId, assetTree),
      (reason) =>
        new Error(`Could not simulate add liquidity with reason: ${reason}`)
    ),
    taskEither.map((expectedLP) => fromChainUnits(expectedLP.toString()))
  )();
}

export const useSimulateAddLiquidity = () => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  return useCallback(
    (
      poolId: string,
      leftAssetWithBalance: AssetWithBalance,
      rightAssetWithBalance: AssetWithBalance
    ) => {
      return pipe(
        option.Do,
        option.bind("account", () => option.fromNullable(selectedAccount)),
        option.bind("api", () => option.fromNullable(parachainApi)),
        option.bind("assetTree", () =>
          getAssetTree(leftAssetWithBalance, rightAssetWithBalance)
        ),
        option.fold(
          async () => new BigNumber(0),
          async ({ api, account, assetTree }) => {
            return pipe(
              await trySimulateAddLiquidity(api, account, poolId, assetTree),
              either.fold(
                () => new BigNumber(0),
                (lp) => lp
              )
            );
          }
        )
      );
    },
    [parachainApi, selectedAccount]
  );
};
