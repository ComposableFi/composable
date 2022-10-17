import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { callbackGate, fromChainIdUnit, fromPerbill, humanDateDiff, toChainIdUnit, unwrapNumberOrHex } from "shared";
import { RewardPool } from "@/stores/defi/polkadot/stakingRewards/slice";
import { Executor, getSigner } from "substrate-react";
import { AnyComponentMap, EnqueueSnackbar, SnackbarKey } from "notistack";
import { APP_NAME } from "@/defi/polkadot/constants";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";

export async function fetchStakingRewardPosition(
  api: ApiPromise,
  fnftCollectionId: BigNumber,
  setter: (position: any) => void
) {
  const result: any =
    await api.query.stakingRewards.stakes(
      api.createType("u128", fnftCollectionId.toString()), null
    );

  if (result.isSome) {
    const data: any = result.toJSON();
    setter({
      unlockPenalty: unwrapNumberOrHex(data.lock.unlockPenalty),
      share: fromChainIdUnit(unwrapNumberOrHex(data.share)),
      stake: fromChainIdUnit(unwrapNumberOrHex(data.stake))
    });
  }
}

export function transformRewardPool(rewardPoolsWrapped: any): RewardPool {
  return {
    owner: rewardPoolsWrapped.owner,
    assetId: rewardPoolsWrapped.assetId.toString(),
    rewards: rewardPoolsWrapped.rewards,
    totalShares: unwrapNumberOrHex(rewardPoolsWrapped.totalShares.toString()),
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
            fromPerbill(multiplier as string)
          ]
        )
      )
    },
    shareAssetId: rewardPoolsWrapped.shareAssetId.toString(),
    financialNftAssetId: rewardPoolsWrapped.financialNftAssetId.toString()
  } as unknown as RewardPool;
}

export async function fetchRewardPools(api: ApiPromise, assetId: number) {
  const rewardPoolsWrapped: any = (
    await api.query.stakingRewards.rewardPools(api.createType("u128", assetId))
  ).toJSON();

  if (!rewardPoolsWrapped) return null;

  return transformRewardPool(rewardPoolsWrapped);
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
  closeSnackbar
}: {
  executor: Executor | undefined,
  parachainApi: ApiPromise | undefined,
  account: { name: string; address: string } | undefined,
  assetId: number,
  lockablePICA: BigNumber,
  lockPeriod: string,
  enqueueSnackbar: EnqueueSnackbar<AnyComponentMap>,
  closeSnackbar: (key?: SnackbarKey) => void
}) {
  return callbackGate(async (executor, api, account) => {
    let snackbarKey: SnackbarKey | undefined;
    const signer = await getSigner(APP_NAME, account.address);
    await executor.execute(
      api.tx.stakingRewards.stake(
        assetId.toString(),
        api.createType(
          "u128",
          toChainIdUnit(lockablePICA).toString()
        ),
        api.createType("u64", lockPeriod.toString())
      ),
      account.address,
      api,
      signer,
      (txHash: string) => {
        snackbarKey = enqueueSnackbar("Processing stake on the chain", {
          variant: "info",
          isClosable: true,
          persist: true,
          url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash
        });
      },
      (txHash: string) => {
        closeSnackbar(snackbarKey);
        enqueueSnackbar(
          `Successfully staked ${lockablePICA
            .toFixed()
            .toString()} PICA`,
          {
            variant: "success",
            isClosable: true,
            persist: true,
            url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash
          }
        );
      },
      (errorMessage: string) => {
        closeSnackbar(snackbarKey);
        enqueueSnackbar(
          "An error occurred while processing transaction",
          {
            variant: "error",
            isClosable: true,
            persist: true,
            description: errorMessage
          }
        );
      }
    );
  }, executor, parachainApi, account);
}

export function calculateStakingPeriodAPR(lockPeriod: string, durationPresets: {
  [key in string]: BigNumber
}) {
  if (!lockPeriod) {
    return 0;
  }
  const SECONDS_IN_YEAR = 31536000;
  const APR = durationPresets[lockPeriod].multipliedBy(SECONDS_IN_YEAR / Number(lockPeriod));

  return APR.toFixed(2);
}
