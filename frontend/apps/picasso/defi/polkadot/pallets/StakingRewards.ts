import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import {
  callbackGate,
  fromChainIdUnit,
  fromPerbill,
  humanDateDiff,
  subscanExtrinsicLink,
  toChainIdUnit,
  unwrapNumberOrHex,
} from "shared";
import { RewardPool } from "@/stores/defi/polkadot/stakingRewards/slice";
import { Executor } from "substrate-react";
import { AnyComponentMap, EnqueueSnackbar, SnackbarKey } from "notistack";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { Signer } from "@polkadot/api/types";
import { pipe } from "fp-ts/lib/function";
import * as E from "fp-ts/Either";
import * as TE from "fp-ts/TaskEither";
import { tryCatch } from "fp-ts/TaskEither";

export async function fetchStakingRewardPosition(
  api: ApiPromise,
  fnftCollectionId: BigNumber,
  setter: (position: any) => void
) {
  const result: any = await api.query.stakingRewards.stakes(
    api.createType("u128", fnftCollectionId.toString()),
    null
  );

  if (result.isSome) {
    const data: any = result.toJSON();
    setter({
      unlockPenalty: unwrapNumberOrHex(data.lock.unlockPenalty),
      share: fromChainIdUnit(unwrapNumberOrHex(data.share)),
      stake: fromChainIdUnit(unwrapNumberOrHex(data.stake)),
    });
  }
}

export function transformRewardPool(rewardPoolsWrapped: any): RewardPool {
  console.log(rewardPoolsWrapped);
  return {
    owner: rewardPoolsWrapped.owner,
    // assetId: rewardPoolsWrapped.assetId.toString(), assetId is removed
    rewards: rewardPoolsWrapped.rewards,
    // totalShares: unwrapNumberOrHex(rewardPoolsWrapped.totalShares.toString()), total shares is removed
    claimedShares: unwrapNumberOrHex(
      rewardPoolsWrapped.claimedShares.toString()
    ),
    endBlock: unwrapNumberOrHex(rewardPoolsWrapped.endBlock.toString()),
    lock: {
      ...rewardPoolsWrapped.lock,
      durationPresets: Object.fromEntries(
        Object.entries(rewardPoolsWrapped.lock.durationPresets).map(
          ([duration, multiplier]) => [
            duration,
            fromPerbill(multiplier as string),
          ]
        )
      ),
    },
    shareAssetId: rewardPoolsWrapped.shareAssetId.toString(),
    financialNftAssetId: rewardPoolsWrapped.financialNftAssetId.toString(),
  } as unknown as RewardPool;
}

export async function fetchRewardPools(api: ApiPromise, assetId: number) {
  const getRewardPools = tryCatch(
    () => api.query.stakingRewards.rewardPools(api.createType("u128", assetId)),
    () => new Error("Could not query reward pools")
  );

  const task: TE.TaskEither<Error, RewardPool> = pipe(
    getRewardPools,
    TE.chainW((e) =>
      e.isSome
        ? TE.right(transformRewardPool(e.toJSON()))
        : TE.left(new Error("Empty result from reward pool"))
    )
  );

  return pipe(
    await task(),
    E.fold(
      () => null,
      (a) => a
    )
  )
}

export function formatDurationOption(duration: string, multiplier: BigNumber) {
  const future = new Date();
  future.setSeconds(future.getSeconds() + parseInt(duration));
  const [diff, label] = humanDateDiff(new Date(), future);

  return `${diff} ${label} (${multiplier.toFixed(2).toString()}%)`;
}

export type DurationOption = {
  [key in number]: string;
};

export function stake({
  executor,
  parachainApi,
  account,
  assetId,
  lockablePICA,
  lockPeriod,
  enqueueSnackbar,
  closeSnackbar,
  signer,
}: {
  executor: Executor | undefined;
  parachainApi: ApiPromise | undefined;
  account: InjectedAccountWithMeta | undefined;
  assetId: number;
  lockablePICA: BigNumber;
  lockPeriod: string;
  enqueueSnackbar: EnqueueSnackbar<AnyComponentMap>;
  closeSnackbar: (key?: SnackbarKey) => void;
  signer: Signer | undefined;
}) {
  return callbackGate(
    async (executor, api, account, _signer) => {
      let snackbarKey: SnackbarKey | undefined;
      await executor.execute(
        api.tx.stakingRewards.stake(
          assetId.toString(),
          api.createType("u128", toChainIdUnit(lockablePICA).toString()),
          api.createType("u64", lockPeriod.toString())
        ),
        account.address,
        api,
        _signer,
        (txHash: string) => {
          snackbarKey = enqueueSnackbar("Processing stake on the chain", {
            variant: "info",
            isClosable: true,
            persist: true,
            url: subscanExtrinsicLink("picasso", txHash),
          });
        },
        (txHash: string) => {
          closeSnackbar(snackbarKey);
          enqueueSnackbar(
            `Successfully staked ${lockablePICA.toFixed().toString()} PICA`,
            {
              variant: "success",
              isClosable: true,
              persist: true,
              url: subscanExtrinsicLink("picasso", txHash),
            }
          );
        },
        (errorMessage: string) => {
          closeSnackbar(snackbarKey);
          enqueueSnackbar("An error occurred while processing transaction", {
            variant: "error",
            isClosable: true,
            persist: true,
            description: errorMessage,
          });
        }
      );
    },
    executor,
    parachainApi,
    account,
    signer
  );
}

export function calculateStakingPeriodAPR(
  lockPeriod: string,
  durationPresets: {
    [key in string]: BigNumber;
  }
) {
  if (!lockPeriod) {
    return 0;
  }
  const SECONDS_IN_YEAR = 31536000;
  const APR = durationPresets[lockPeriod].multipliedBy(
    SECONDS_IN_YEAR / Number(lockPeriod)
  );

  return APR.toFixed(2);
}
