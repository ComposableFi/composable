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

const processedTransactions: string[] = [];
const Updater = () => {
  const { putAssetBalance, supportedAssets } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const extrinsicCalls = useExtrinsics();

  const updateAllBalances = useCallback(async () => {
    if (parachainApi && selectedAccount) {
      let assets = [];
      for (let index = 0; index < supportedAssets.length; index++) {
        assets.push(supportedAssets[index].network[DEFAULT_NETWORK_ID])
        const balance = await fetchBalanceByAssetId(
          parachainApi,
          selectedAccount.address,
          supportedAssets[index].network[DEFAULT_NETWORK_ID]
        )
        putAssetBalance(DEFAULT_NETWORK_ID, supportedAssets[index].network[DEFAULT_NETWORK_ID], balance);
      }
    }
  }, [parachainApi, selectedAccount, supportedAssets, putAssetBalance])

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
          tx.sender === selectedAccount.address &&
          tx.status === "isFinalized" &&
          (tx.section === "dexRouter" || tx.section === "pablo") &&
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
