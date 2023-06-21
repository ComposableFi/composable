import { readonlyArray } from "fp-ts";
import { PoolConfig } from "@/store/createPool/types";
import BigNumber from "bignumber.js";
import { flow, pipe } from "fp-ts/function";
import {
  addLiquidityToPoolViaPablo,
  DEFAULT_NETWORK_ID,
  fromChainUnits,
} from "@/defi/utils";
import { AssetRatio, SubstrateNetworkId } from "shared";
import { InputConfig } from "@/components/Organisms/liquidity/AddForm/types";
import * as O from "fp-ts/Option";
import { ApiPromise } from "@polkadot/api";
import { Executor } from "substrate-react";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { Signer } from "@polkadot/api/types";
import { TaskEither, tryCatch } from "fp-ts/TaskEither";
import { RuntimeDispatchInfo } from "@polkadot/types/interfaces/payment";
import * as E from "fp-ts/Either";

export function getInputConfig(
  pool: O.Option<PoolConfig>,
  getTokenBalance: (
    tokenId: string,
    network: "kusama" | "picasso" | "karura" | "statemine"
  ) => { free: BigNumber; locked: BigNumber }
): O.Option<InputConfig[]> {
  return pipe(
    pool,
    O.map((p) => p.config.assets),
    O.map(
      flow(
        readonlyArray.fromArray,
        readonlyArray.map((asset) => ({
          asset,
          chainId: DEFAULT_NETWORK_ID as SubstrateNetworkId,
          balance: getTokenBalance(
            asset.getPicassoAssetId() as string,
            DEFAULT_NETWORK_ID
          ),
        })),
        readonlyArray.toArray
      )
    )
  );
}

export function getAssetOptions(config: InputConfig[]) {
  return [
    {
      value: "none",
      label: "Select",
      icon: undefined,
      disabled: true,
      hidden: true,
    },
    ...Object.values(config).map((item) => ({
      value: item.asset.getIdOnChain(DEFAULT_NETWORK_ID) as string,
      label: item.asset.getSymbol(),
      icon: item.asset.getIconUrl(),
    })),
  ];
}

export function getPaymentInfoCall(
  assetTree: O.Option<{ [p: number]: string }>,
  parachainApi: ApiPromise,
  poolId: string,
  executor: Executor,
  account: InjectedAccountWithMeta,
  signer: Signer
): O.Option<TaskEither<Error, RuntimeDispatchInfo>> {
  return pipe(
    assetTree,
    O.map((assets) => addLiquidityToPoolViaPablo(parachainApi, poolId, assets)),
    O.map((call) =>
      tryCatch(
        () => executor.paymentInfo(call, account.address, signer),
        () => new Error("Could not fetch payment info")
      )
    )
  );
}

export function parseRuntimeInfo(
  result: E.Either<Error, RuntimeDispatchInfo>,
  gasFeeRatio: AssetRatio | null,
  gasFeeTokenDecimals: number
) {
  return pipe(
    result,
    E.match(
      () => new BigNumber(0),
      (runtimeInfo) =>
        pipe(
          gasFeeRatio,
          O.fromNullable,
          O.fold(
            () => fromChainUnits(runtimeInfo.partialFee.toString()),
            (ratio) =>
              fromChainUnits(
                new BigNumber(runtimeInfo.partialFee.toString())
                  .multipliedBy(ratio.n)
                  .div(ratio.d),
                gasFeeTokenDecimals
              )
          )
        )
    )
  );
}
