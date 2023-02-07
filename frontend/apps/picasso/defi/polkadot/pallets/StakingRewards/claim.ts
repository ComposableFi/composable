import { ApiPromise } from "@polkadot/api";
import { pipe } from "fp-ts/lib/function";
import { Executor } from "substrate-react";
import * as O from "fp-ts/Option";
import * as A from "fp-ts/Array";
import { Signer } from "@polkadot/api/types";
import { useStore } from "@/stores/root";

export async function claimAllPicaRewards(
  api: ApiPromise | undefined,
  executor: Executor | undefined,
  signer: Signer | undefined,
  address: string | undefined,
  onReady: (txHash: string) => void,
  onFinalize: (txHash: string) => void,
  onError: (error: string) => void
) {
  pipe(
    prepSignedTx(api, executor, signer, address),
    O.bindTo("context"),
    O.bind("fnfts", getClaimableFnfts()),
    O.bind("txs", ({ context, fnfts }) =>
      pipe(
        fnfts,
        A.map(([collectionId, instanceId]) =>
          context.api.tx.stakingRewards.claim(collectionId, instanceId)
        ),
        O.fromPredicate((txs) => txs.length > 0)
      )
    ),
    O.map(({ context, txs }) => {
      context.executor.execute(
        context.api.tx.utility.batch(txs),
        context.address,
        context.api,
        context.signer,
        onReady,
        onFinalize,
        onError
      );
    }),
    O.getOrElse(() =>
      console.log("Claim did not executed because not all params are ready")
    )
  );
}

function prepSignedTx(
  api: ApiPromise | undefined,
  executor: Executor | undefined,
  signer: Signer | undefined,
  address: string | undefined
) {
  return pipe(
    O.Do,
    O.bind("api", () => O.fromNullable(api)),
    O.bind("executor", () => O.fromNullable(executor)),
    O.bind("signer", () => O.fromNullable(signer)),
    O.bind("address", () => O.fromNullable(address))
  );
}

function getClaimableFnfts() {
  return () =>
    pipe(
      Object.entries(useStore.getState().claimableRewards),
      A.filter(([_, rewards]) =>
        rewards.some(
          (reward) =>
            reward.assetId ===
            String(
              useStore.getState().substrateTokens.tokens.pica.chainId.picasso
            )
        )
      ),
      A.map(([fnft]) => fnft.split("::")),
      O.fromPredicate((item) => item.length > 0)
    );
}
