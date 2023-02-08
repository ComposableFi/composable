import { ApiPromise } from "@polkadot/api";
import { useStore } from "@/stores/root";
import BigNumber from "bignumber.js";
import config from "@/constants/config";
import { getClaimable } from "@/defi/polkadot/pallets/StakingRewards/rpc";
import { isPalletSupported } from "shared";

let count = 0;

function getRewardKey(collectionId: string, instanceId: string) {
  const rewardKey = [collectionId, instanceId].join("::");
  return rewardKey;
}

async function updateClaimableAmount(api: ApiPromise) {
  const stakingPortfolio = useStore.getState().stakingPortfolio;
  if (
    stakingPortfolio.length === 0 ||
    !isPalletSupported(api)("StakingRewards")
  )
    return;
  // Reset because we are fetching a new claimable for all assets.
  useStore.getState().resetClaimableRewards();
  let list = [];

  for (const item of stakingPortfolio) {
    const { collectionId, instanceId } = item;
    list.push(getClaimable(api, collectionId, instanceId));
  }

  // TODO: Below should be removed once claimable RPC is working with real data
  if (config.stakingRewards.demoMode) {
    useStore.getState().setClaimableRewards("1001::123", {
      assetId: "1",
      balance: BigNumber(count++),
    });
  }

  const claimableList = await Promise.all(list);

  for (const claimable of claimableList) {
    if (claimable.result.isOk) {
      useStore.setState((state) => {
        state.claimableRewards[rewardKey] = [];
      });
      const rewardKey = getRewardKey(
        claimable.collectionId,
        claimable.instanceId
      );
      for (let [assetId, balance] of claimable.result.asOk.entries()) {
        useStore.getState().setClaimableRewards(rewardKey, {
          assetId: assetId.toString(),
          balance: new BigNumber(balance.toString()),
        });
      }
    }
  }
}

export async function subscribeClaimableRewards(api: ApiPromise | undefined) {
  if (!api) return;
  // If we have staking portfolio, listen to new blocks
  // on each block call claimable RPC to fetch claimable for all portfolios
  const unsub = await api.query.system.number(() => updateClaimableAmount(api));

  return () => {
    console.log("[claimable]: Unsubscribing from new blocks");
    unsub();
  };
}
