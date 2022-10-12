import { useParachainApi, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID, fetchOwnedFinancialNfts } from "@/defi/utils";
import { fetchStakingRewardPools } from "@/defi/utils/stakingRewards";
import { useOnChainAssetIds } from "@/store/hooks/useOnChainAssetsIds";
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
import { useTotalXTokensIssued } from "@/defi/hooks/stakingRewards/useTotalXTokensIssued";

export function updateStakingRewardPool(
  api: ApiPromise,
  assetId: string
): void {
  fetchStakingRewardPools(api, [assetId]).then(pools => {
    if (pools.length > 0) {
      putStakingRewardPool(pools[0])
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
  fetchOwnedFinancialNfts(parachainApi, address)
    .then(setOwnedFinancialNfts)
    .catch(resetOwnedFinancialNfts);
}

const Updater = () => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const onChainAssetIds = useOnChainAssetIds();
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  useEffect(() => {
    if (parachainApi && onChainAssetIds.size > 0) {
      updateStakingRewardPools(parachainApi, Array.from(onChainAssetIds));
    }
  }, [parachainApi, onChainAssetIds]);

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

  return null;
};

export default Updater;
