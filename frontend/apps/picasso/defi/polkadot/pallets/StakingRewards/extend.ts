import { toChainIdUnit } from "shared";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { Signer } from "@polkadot/api/types";
import { Executor } from "substrate-react";

export function extend(
  api: ApiPromise,
  signer: Signer,
  address: string,
  executor: Executor,
  onReady: (txHash: string) => void,
  onFinalize: (txHash: string) => void,
  onError: (error: string) => void
) {
  return function (
    extendAmount: number | BigNumber,
    fnftCollectionId: string,
    fnftInstanceId: string
  ) {
    return executor.execute(
      api.tx.stakingRewards.extend(
        fnftCollectionId,
        fnftInstanceId,
        api.createType("u128", toChainIdUnit(extendAmount).toString())
      ),
      address,
      api,
      signer,
      onReady,
      onFinalize,
      onError
    );
  };
}
