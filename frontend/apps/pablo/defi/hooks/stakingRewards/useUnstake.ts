import {
  getSigner,
  useExecutor,
  useParachainApi,
  useSelectedAccount,
} from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useCallback } from "react";
import { useSnackbar } from "notistack";
import { APP_NAME } from "@/defi/polkadot/constants";
import _ from "lodash";
import BigNumber from "bignumber.js";

export type StakeProps = {
  financialNftCollectionId?: BigNumber;
  financialNftInstanceId?: BigNumber;
};

export function useUnstake({
  financialNftCollectionId,
  financialNftInstanceId,
}: StakeProps) {
  const { enqueueSnackbar } = useSnackbar();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const executor = useExecutor();

  return useCallback(async () => {
    if (
      !parachainApi ||
      !selectedAccount ||
      !executor ||
      !financialNftCollectionId ||
      !financialNftInstanceId
    ) {
      throw new Error("Invalid staking position.");
    }

    try {
      const signer = await getSigner(APP_NAME, selectedAccount.address);
      await executor.execute(
        parachainApi.tx.stakingRewards.unstake(
          parachainApi.createType("u128", financialNftCollectionId.toString()),
          parachainApi.createType("u128", financialNftInstanceId.toString())
        ),
        selectedAccount.address,
        parachainApi,
        signer,
        (_transactionHash) => {
          console.log("Tx Ready: ", _transactionHash);
        },
        (_transactionHash, _events) => {
          enqueueSnackbar(
            `Position Unstaked, transaction hash: ${_transactionHash}`,
            {
              variant: "success",
            }
          );
        },
        (errorMessage) => {
          enqueueSnackbar(`Error: ${errorMessage}`, {
            variant: "error",
          });
        }
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
    enqueueSnackbar,
    financialNftCollectionId,
    financialNftInstanceId,
  ]);
}
