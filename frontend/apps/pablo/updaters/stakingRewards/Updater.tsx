import { useParachainApi, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID, PBLO_ASSET_ID } from "@/defi/utils";
import { fetchStakingRewardPools } from "@/defi/utils/stakingRewards";
import { fetchStakingPositionHistory } from "@/defi/subsquid/stakingRewards/queries";
import { resetOwnedFinancialNfts, setOwnedFinancialNfts } from "@/store/financialNfts/financialNfts.slice";
import { ApiPromise } from "@polkadot/api";
import { useEffect } from "react";
import {
  putStakingRewardPool,
  putStakingRewardPools,
  putStakingRewardPoolStakedPositionsHistory,
  resetStakingRewardPools,
  resetStakingRewardPoolStakedPositionsHistory,
} from "@/store/stakingRewards/stakingRewards.slice";
import { useAllLpTokenRewardingPools } from "@/defi/hooks/useAllLpTokenRewardingPools";
import { useAsyncEffect } from "@/hooks/useAsyncEffect";
import { FinancialNft } from "shared";

export function updateStakingRewardPool(
  api: ApiPromise,
  assetId: string
): void {
  fetchStakingRewardPools(api, [assetId]).then(pools => {
    if (pools.length > 0) {
      putStakingRewardPool(pools[0].pool, assetId)
    }
  })
}

export function updateStakingRewardPools(
  parachainApi: ApiPromise,
  assetIds: string[]
): void {
  fetchStakingRewardPools(parachainApi, assetIds)
    .then(putStakingRewardPools)
    .catch(resetStakingRewardPools);
}

export function updateStakingPositionsHistory(address: string): void {
  fetchStakingPositionHistory(address)
    .then(putStakingRewardPoolStakedPositionsHistory)
    .catch(resetStakingRewardPoolStakedPositionsHistory);
}

export function updateOwnedFinancialNfts(
  parachainApi: ApiPromise,
  address: string
): void {
  FinancialNft.ownedFinancialNfts(parachainApi, address)
    .then(setOwnedFinancialNfts)
    .catch(resetOwnedFinancialNfts);
}

const Updater = () => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  useEffect(() => {
    if (parachainApi) {
      updateStakingRewardPools(parachainApi, [PBLO_ASSET_ID]);
    }
  }, [parachainApi]);

  useEffect(() => {
    if (selectedAccount) {
      updateStakingPositionsHistory(selectedAccount.address);
    }
  }, [selectedAccount]);

  useEffect(() => {
    if (parachainApi && selectedAccount) {
      updateOwnedFinancialNfts(parachainApi, selectedAccount.address);
    }
  }, [parachainApi, selectedAccount]);

  const lpRewardingPools = useAllLpTokenRewardingPools();
  useAsyncEffect(async (): Promise<void> => {
    if (lpRewardingPools.length > 0) {
      for (const lpRewardingPool of lpRewardingPools) {
        updateStakingRewardPool(
          lpRewardingPool.getApi(),
          lpRewardingPool.getLiquidityProviderToken().getPicassoAssetId() as string
        )
      }
    }
  }, [lpRewardingPools])

  return null;
};

export default Updater;
