import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import {
  callbackGate,
  fromChainIdUnit,
  fromPerbill,
  subscanExtrinsicLink,
  toChainIdUnit,
  unwrapNumberOrHex,
} from "shared";
import {
  PortfolioItem,
  RewardPool,
} from "@/stores/defi/polkadot/stakingRewards/slice";
import { Executor } from "substrate-react";
import { AnyComponentMap, EnqueueSnackbar, SnackbarKey } from "notistack";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { Signer } from "@polkadot/api/types";
import { pipe } from "fp-ts/lib/function";
import * as TE from "fp-ts/TaskEither";
import { tryCatch } from "fp-ts/TaskEither";
import { StakingPosition } from "@/apollo/queries/stakingPositions";
import config from "@/constants/config";
import { ComposableTraitsStakingRewardPool } from "@/../../packages/defi-interfaces";
import { Option } from "@polkadot/types-codec";
import { useStore } from "@/stores/root";
import { useStakeForm } from "@/components/Organisms/Staking/stakeFormStore";

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
    // endBlock: unwrapNumberOrHex(rewardPoolsWrapped.endBlock.toString()),
    lock: {
      unlockPenalty: rewardPoolsWrapped.lock.unlockPenalty,
      durationPresets: Object.fromEntries(
        Object.entries(rewardPoolsWrapped.lock.durationMultipliers.presets).map(
          ([duration, multiplier]) => [
            duration,
            fromPerbill(multiplier as string),
          ]
        )
      ),
    },
    shareAssetId: rewardPoolsWrapped.shareAssetId.toString(),
    financialNftAssetId: rewardPoolsWrapped.financialNftAssetId.toString(),
    minimumStakingAmount: fromChainIdUnit(
      rewardPoolsWrapped.minimumStakingAmount.toString()
    ),
  } as unknown as RewardPool;
}

export function transformStakingPortfolio(
  position: StakingPosition,
  result: any,
  rewardPools: {
    [assetId: string]: RewardPool;
  },
  assetId: string
): PortfolioItem {
  return {
    collectionId: position.fnftCollectionId,
    instanceId: position.fnftInstanceId,
    assetId: position.assetId,
    endTimestamp: position.endTimestamp.toString(),
    id: position.id,
    unlockPenalty: fromPerbill(rewardPools[assetId].lock.unlockPenalty),
    multiplier: rewardPools[assetId].lock.durationPresets[result.lock.duration],
    share: fromChainIdUnit(unwrapNumberOrHex(result.share)),
    stake: fromChainIdUnit(unwrapNumberOrHex(result.stake)),
    shareAssetId: rewardPools[assetId].shareAssetId.toString(),
  };
}

export function tryFetchStakes(api: ApiPromise, fnftCollectionId: string) {
  const getStakes = tryCatch(
    () => api.query.stakingRewards.stakes.entries(fnftCollectionId),
    () => "0"
  );
  return pipe(getStakes);
}

export function tryFetchStakePortfolio(
  api: ApiPromise,
  position: StakingPosition,
  rewardPools: { [assetId: string]: RewardPool },
  assetId: string
) {
  const getStakePortfolio = tryCatch(
    () =>
      api.query.stakingRewards.stakes(
        api.createType("u128", position.fnftCollectionId),
        api.createType("u64", position.fnftInstanceId)
      ),
    () =>
      new Error(
        `Could not fetch stakes for Position ${position.fnftCollectionId}:${position.fnftInstanceId}`
      )
  );
  return pipe(
    getStakePortfolio,
    TE.chainW((stakingPortfolio) =>
      stakingPortfolio.isSome
        ? TE.right(
            transformStakingPortfolio(
              position,
              stakingPortfolio.toJSON(),
              rewardPools,
              assetId
            )
          )
        : TE.left(new Error("Empty staking portfolio"))
    )
  );
}

async function getMockPortfolio(): Promise<
  Option<ComposableTraitsStakingRewardPool>
> {
  return {
    isSome: true,
    toJSON: () => config.stakingRewards.picaRewardPools,
  } as unknown as Option<ComposableTraitsStakingRewardPool>;
}

export function tryFetchRewardPool(
  api: ApiPromise,
  assetId: string | number
): TE.TaskEither<Error, RewardPool> {
  const getRewardPools = tryCatch(
    () =>
      config.stakingRewards.demoMode
        ? getMockPortfolio()
        : api.query.stakingRewards.rewardPools(api.createType("u128", assetId)),
    () => new Error("Could not query reward pools")
  );

  return pipe(
    getRewardPools,
    TE.chainW((e) =>
      e.isSome
        ? TE.right(transformRewardPool(e.toJSON()))
        : TE.left(new Error("Empty result from reward pool"))
    )
  );
}

export type DurationOption = {
  [key in number]: string;
};

function onReady(
  enqueueSnackbar: EnqueueSnackbar<AnyComponentMap>
): (txHash: string) => string | number {
  return (txHash: string) => {
    return enqueueSnackbar("Processing stake on the chain", {
      variant: "info",
      isClosable: true,
      persist: true,
      url: subscanExtrinsicLink("picasso", txHash),
    });
  };
}

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
  executor: Executor;
  parachainApi: ApiPromise;
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
        (txHash) => {
          snackbarKey = onReady(enqueueSnackbar)(txHash);
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
          useStakeForm.setState((state) => {
            state.amount = new BigNumber(0);
            state.lockPeriod = "0";
          });
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
  rewardPool: RewardPool,
  assetId: string,
  multiplier: number
) {
  // rewards per sec * (365* 24* 60* 60) / pool’s shares
  const SECONDS_IN_YEAR = 365 * 24 * 60 * 60;
  const currentAmount = useStakeForm.getState().amount;
  const rewardsPerSec = rewardPool.rewards[assetId].rewardRate.amount;
  const poolShare = new BigNumber(useStore.getState().maximumPicaShares);
  const myShare = currentAmount.multipliedBy(multiplier);
  const apr = new BigNumber(fromChainIdUnit(rewardsPerSec.toString()))
    .multipliedBy(SECONDS_IN_YEAR)
    .div(myShare.plus(poolShare));

  if (apr.isNaN() || !apr.isFinite()) {
    return "0";
  }

  return apr.toFormat(2);
}
