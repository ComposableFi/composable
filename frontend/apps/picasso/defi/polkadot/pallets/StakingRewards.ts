import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { Option } from "@polkadot/types-codec";
import {
  fromChainIdUnit,
  fromPerbill,
  humanDateDiff,
  unwrapNumberOrHex,
} from "shared";
import { ComposableTraitsStakingStake } from "defi-interfaces";
import { RewardPool } from "@/stores/defi/polkadot/stakingRewards/slice";

export async function fetchStakingRewardPosition(
  api: ApiPromise,
  positionId: BigNumber,
  setter: (position: any) => void
) {
  const result: Option<ComposableTraitsStakingStake> =
    await api.query.stakingRewards.stakes(
      api.createType("u128", positionId.toString())
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
