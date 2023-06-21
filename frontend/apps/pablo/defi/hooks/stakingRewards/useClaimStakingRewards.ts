import {
  useExecutor,
  useParachainApi,
  useSelectedAccount,
  useSigner,
} from "substrate-react";
import _ from "lodash";
import { useCallback, useMemo } from "react";
import { useSnackbar } from "notistack";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import {
  transactionStatusSnackbarMessage,
  SNACKBAR_TYPES,
} from "../addLiquidity/useAddLiquidity";
import { updateStake } from "@/store/stakingRewards/stakingRewards.slice";
import { decodeStake } from "@/defi/utils/stakingRewards";
import { updateStakingRewardPool } from "@/updaters/stakingRewards/Updater";

const TxOrigin = "Claim Staking Position";

export type StakeClaimProps = {
  financialNftCollectionId?: string;
  financialNftInstanceId?: string;
  principalAssetId?: string;
};

export function useClaimStakingRewards({
  financialNftCollectionId,
  financialNftInstanceId,
  principalAssetId
}: StakeClaimProps) {
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
    (transactionHash: string, _eventsRecord: any[]) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage(
          TxOrigin,
          transactionHash,
          "Finalized"
        ),
        SNACKBAR_TYPES.SUCCESS
      );

      if (parachainApi && principalAssetId) {
        updateStakingRewardPool(parachainApi, principalAssetId);        
      }
      if (parachainApi && financialNftCollectionId && financialNftInstanceId) {
        parachainApi.query.stakingRewards
          .stakes(
            parachainApi.createType("u128", financialNftCollectionId),
            parachainApi.createType("u64", financialNftInstanceId)
          )
          .then((stake) => {
            updateStake(financialNftCollectionId, decodeStake(stake));
          });
      }
    },
    [enqueueSnackbar, parachainApi, principalAssetId, financialNftCollectionId, financialNftInstanceId]
  );

  const onTxError = useCallback(
    (errorMessage: string) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage(TxOrigin, errorMessage, "Error"),
        SNACKBAR_TYPES.ERROR
      );
    },
    [enqueueSnackbar]
  );

  const collectionId = useMemo(() => {
    if (!parachainApi || !financialNftCollectionId) return null;

    return parachainApi.createType("u128", financialNftCollectionId);
  }, [parachainApi, financialNftCollectionId]);

  const instanceId = useMemo(() => {
    if (!parachainApi || !financialNftInstanceId) return null;

    return parachainApi.createType("u64", financialNftInstanceId);
  }, [parachainApi, financialNftInstanceId]);

  return useCallback(async () => {
    if (
      !parachainApi ||
      !selectedAccount ||
      !executor ||
      !collectionId ||
      !instanceId ||
      !signer
    ) {
      throw new Error("Invalid stake pool.");
    }

    try {
      await executor.execute(
        parachainApi.tx.stakingRewards.claim(collectionId, instanceId),
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
    executor,
    collectionId,
    instanceId,
    signer,
    onTxReady,
    onTxFinalized,
    onTxError,
  ]);
}
