import {
  useExecutor,
  useParachainApi,
  useSelectedAccount,
  useSigner,
} from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useCallback } from "react";
import { useSnackbar } from "notistack";
import _ from "lodash";
import BigNumber from "bignumber.js";
import {
  transactionStatusSnackbarMessage,
  SNACKBAR_TYPES,
} from "../addLiquidity/useAddLiquidity";
import { updateOwnedFinancialNfts } from "@/updaters/stakingRewards/Updater";

export type UnstakeProps = {
  financialNftCollectionId?: BigNumber;
  financialNftInstanceId?: BigNumber;
};

const TxOrigin = "Burn Staking Position";

export function useUnstake({
  financialNftCollectionId,
  financialNftInstanceId,
}: UnstakeProps) {
  const { enqueueSnackbar } = useSnackbar();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const signer = useSigner();
  const executor = useExecutor();

  const onTxReady = useCallback(
    (transactionHash: string) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage(
          TxOrigin,
          transactionHash,
          "Initiated"
        ),
        SNACKBAR_TYPES.INFO
      );
    },
    [enqueueSnackbar]
  );

  const onTxFinalized = useCallback(
    (transactionHash: string, _eventRecords: any[]) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage(
          TxOrigin,
          transactionHash,
          "Finalized"
        ),
        SNACKBAR_TYPES.SUCCESS
      );

      if (parachainApi && selectedAccount) {
        updateOwnedFinancialNfts(parachainApi, selectedAccount.address);
      }
    },
    [enqueueSnackbar,  parachainApi, selectedAccount]
  );

  const onTxError = useCallback(
    (transactionError: string) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage(TxOrigin, transactionError, "Error"),
        SNACKBAR_TYPES.ERROR
      );
    },
    [enqueueSnackbar]
  );

  return useCallback(async () => {
    try {
      if (
        !parachainApi ||
        !selectedAccount ||
        !executor ||
        !signer ||
        !financialNftCollectionId ||
        !financialNftInstanceId
      ) {
        throw new Error("Invalid staking position.");
      }
      await executor.execute(
        parachainApi.tx.stakingRewards.unstake(
          parachainApi.createType("u128", financialNftCollectionId.toString()),
          parachainApi.createType("u128", financialNftInstanceId.toString())
        ),
        selectedAccount.address,
        parachainApi,
        signer,
        onTxReady,
        onTxFinalized,
        onTxError
      );
    } catch (error: any) {
      enqueueSnackbar(`Error: ${error.message}`, {
        variant: "error",
      });
    }
  }, [
    parachainApi,
    selectedAccount,
    executor,
    financialNftCollectionId,
    financialNftInstanceId,
    onTxReady,
    onTxFinalized,
    onTxError,
    enqueueSnackbar,
    signer,
  ]);
}
