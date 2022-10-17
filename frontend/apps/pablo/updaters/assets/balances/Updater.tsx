import { useCallback, useEffect } from "react";
import {
  useExtrinsics,
  useParachainApi,
  useSelectedAccount,
} from "substrate-react";
import useStore from "@/store/useStore";
import { fetchBalanceByAssetId } from "@/defi/utils";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import _ from "lodash";
import { useOnChainAssetIds } from "@/store/hooks/useOnChainAssetsIds";

function shouldUpdateBalances(tx: any, account: string): boolean {
  if (
    [
      "dexRouter",
      "bondedFinance",
      "pablo",
      "stakingRewards"
    ].includes(tx.section) && tx.sender === account &&
    tx.status === "isFinalized"
  ) {
    return true;
  }
  return false;
}

const processedTransactions: string[] = [];
const Updater = () => {
  const { putAssetBalance } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const onChainAssetIds = useOnChainAssetIds();
  const extrinsicCalls = useExtrinsics();

  const updateAllBalances = useCallback(async () => {
    if (parachainApi && selectedAccount) {
      let assets = Array.from(onChainAssetIds);
      for (let index = 0; index < assets.length; index++) {
        const balance = await fetchBalanceByAssetId(
          parachainApi,
          selectedAccount.address,
          assets[index]
        )
        putAssetBalance(DEFAULT_NETWORK_ID, assets[index], balance);
      }
    }
  }, [parachainApi, selectedAccount, onChainAssetIds, putAssetBalance])

  useEffect(() => {
    if (updateAllBalances && typeof updateAllBalances === "function") {
      updateAllBalances();
    }
  }, [updateAllBalances]);

  useEffect(() => {
    if (
      parachainApi &&
      selectedAccount &&
      Object.values(extrinsicCalls).length > 0
    ) {
      const txs = Object.values(extrinsicCalls);

      let shouldUpdate: string | null = null;
      txs.forEach((tx) => {
        if (
          shouldUpdateBalances(tx, selectedAccount.address) &&
          !processedTransactions.includes(tx.hash)
        ) {
          shouldUpdate = tx.hash;
        }
      });

      if (shouldUpdate !== null) {
        updateAllBalances().then((updatedBalancesAssetList) => {
          processedTransactions.push(shouldUpdate as string);
        });
      }
    }
  }, [extrinsicCalls, parachainApi, selectedAccount, updateAllBalances]);

  return null;
};

export default Updater;