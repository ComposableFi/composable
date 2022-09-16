import { useParachainApi, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID, fetchOwnedFinancialNfts } from "@/defi/utils";
import { fetchStakingRewardPools } from "@/defi/utils/stakingRewards";
import { useOnChainAssetIds } from "@/store/hooks/useOnChainAssetsIds";
import { useEffect } from "react";
import { fetchStakingPositionHistory } from "@/defi/subsquid/stakingRewards/queries";
import { setOwnedFinancialNfts } from "@/store/financialNfts/financialNfts.slice";
import {
  putStakingRewardPools,
  putStakingRewardPoolStakedPositions,
} from "@/store/stakingRewards/stakingRewards.slice";
import { ApiPromise } from "@polkadot/api";

export function updateStakingRewardPools(
  parachainApi: ApiPromise,
  assetIds: string[]
): void {
  fetchStakingRewardPools(parachainApi, assetIds)
    .then(putStakingRewardPools)
    .catch(console.error);
}

export function updateStakingPositionsHistory(address: string): void {
  fetchStakingPositionHistory(address)
    .then(putStakingRewardPoolStakedPositions)
    .catch(console.error);
}

export function updateOwnedFinancialNfts(
  parachainApi: ApiPromise,
  address: string
): void {
  fetchOwnedFinancialNfts(parachainApi, address)
    .then(setOwnedFinancialNfts)
    .catch(console.error);
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
