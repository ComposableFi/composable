import { ApiPromise } from "@polkadot/api";
import { useStore } from "@/stores/root";
import BigNumber from "bignumber.js";
import config from "@/constants/config";
import { getClaimable } from "@/defi/polkadot/pallets/StakingRewards/rpc";
import { fromChainIdUnit, isPalletSupported } from "shared";
import { getPicassoTokenById } from "@/stores/defi/polkadot/tokens/utils";
import { getFnftKey } from "@/defi/polkadot/pallets/StakingRewards";
import { ClaimableRewards } from "@/stores/defi/polkadot/stakingRewards/slice";

let count = 0;

async function updateClaimableAmount(api: ApiPromise) {
  const stakingPortfolio = useStore.getState().stakingPortfolio;
  if (
    stakingPortfolio.size === 0 ||
    !isPalletSupported(api)("StakingRewards") ||
    !api.rpc.stakingRewards
  ) {
    useStore.setState((state) => {
      state.claimableRewards = {};
    });
    return;
  }

  // Reset because we are fetching a new claimable for all assets.
  let list = [];

  for (const [_, item] of stakingPortfolio.entries()) {
    const { collectionId, instanceId } = item;
    const result = await getClaimable(api, collectionId, instanceId)
    if (result) {
      list.push({ result, collectionId, instanceId })
    }
  }

  const claimableList = await Promise.all(list);
  const draftState: ClaimableRewards = {};

  for (const claimable of claimableList) {
    if (claimable.result.isOk) {
      const rewardKey = getFnftKey(
        claimable.collectionId,
        claimable.instanceId
      );

      for (let [assetId, balance] of claimable.result.asOk.entries()) {
        const asset = getPicassoTokenById(assetId.toString());
        const reward = {
          assetId: assetId.toString(),
          balance: fromChainIdUnit(
            balance.toString(),
            asset?.decimals.picasso ?? 12
          ),
        };

        draftState[rewardKey] = Array.isArray(draftState[rewardKey])
          ? [...draftState[rewardKey], reward]
          : [reward];
      }
    }
  }

  useStore.setState((state) => {
    state.claimableRewards = draftState;
  });

  // TODO: Below should be removed once claimable RPC is working with real data
  if (config.stakingRewards.demoMode) {
    useStore.getState().setClaimableRewards("1001::123", {
      assetId: "1",
      balance: BigNumber(count++),
    });
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
