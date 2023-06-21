import {
  useExecutor,
  useParachainApi,
  useSelectedAccount,
  useSigner,
} from "substrate-react";
import { DEFAULT_NETWORK_ID, toChainUnits } from "@/defi/utils";
import _ from "lodash";
import { useCallback } from "react";
import BigNumber from "bignumber.js";
import { useSnackbar } from "notistack";
import {
  SNACKBAR_TYPES,
  transactionStatusSnackbarMessage,
} from "../addLiquidity/useAddLiquidity";
import {
  updateOwnedFinancialNfts,
  updateStakingPositionsHistory,
} from "@/updaters/stakingRewards/Updater";

export type StakeProps = {
  poolId: BigNumber | undefined; // staking pool Id
  amount: BigNumber; // amount to stake
  durationPreset: BigNumber | undefined; // duration in seconds
};

export function useStake({ poolId, amount, durationPreset }: StakeProps) {
  const { enqueueSnackbar } = useSnackbar();
  const signer = useSigner();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const executor = useExecutor();

  const onTxReady = useCallback(
    (transactionHash: string) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage(
          "Staking ",
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
          "Staking",
          transactionHash,
          "Finalized"
        ),
        SNACKBAR_TYPES.SUCCESS
      );

      if (parachainApi && selectedAccount) {
        /**
         * To update UI with latest stake 
         * positions delay intentionally added
         * as subsquid doesn't process the event
         * immediately it receives, can be shortened
         * to discuss later.
         */
        setTimeout(() => {
          updateOwnedFinancialNfts(parachainApi, selectedAccount.address);
          updateStakingPositionsHistory(selectedAccount.address);
        }, 10_000);
      }
    },
    [enqueueSnackbar, parachainApi, selectedAccount]
  );

  const onTxError = useCallback(
    (transactionError: string) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage("Staking", transactionError, "Error"),
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
        !poolId ||
        !durationPreset ||
        !signer
      ) {
        throw new Error("Invalid stake pool.");
      }

      await executor.execute(
        parachainApi.tx.stakingRewards.stake(
          parachainApi.createType("u128", poolId.toString()),
          parachainApi.createType("u128", toChainUnits(amount).toString()),
          parachainApi.createType("u64", durationPreset.toString())
        ),
        selectedAccount.address,
        parachainApi,
        signer,
        onTxReady,
        onTxFinalized,
        onTxError
      );
    } catch (error: any) {
      onTxError(error.message);
    }
  }, [
    parachainApi,
    selectedAccount,
    durationPreset,
    onTxReady,
    onTxFinalized,
    onTxError,
    executor,
    signer,
    amount,
    poolId,
  ]);
}
