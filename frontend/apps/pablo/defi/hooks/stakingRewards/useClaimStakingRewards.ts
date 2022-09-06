import {
    getSigner,
    useExecutor,
    useParachainApi,
    useSelectedAccount,
  } from "substrate-react";
  import _ from "lodash";
  import { useCallback } from "react";
  import BigNumber from "bignumber.js";
  import { useSnackbar } from "notistack";
import { APP_NAME } from "@/defi/polkadot/constants";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
  
  export type StakeProps = {
    positionId: BigNumber | undefined; // position Id
  };
  
  export function useClaimStakingRewards({ positionId }: StakeProps) {
    const { enqueueSnackbar } = useSnackbar();
    const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
    const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
    const executor = useExecutor();
  
    return useCallback(async () => {
      if (!parachainApi || !selectedAccount || !executor || !positionId) {
          throw new Error('Invalid stake pool.');
      }
  
      try {
        const signer = await getSigner(APP_NAME, selectedAccount.address);
        await executor.execute(
          // @ts-ignore
          parachainApi.tx.stakingRewards.claim(
            parachainApi.createType("u128", positionId),
          ),
          selectedAccount.address,
          parachainApi,
          signer,
          (_transactionHash) => {
            console.log("Tx Ready: ", _transactionHash);
          },
          (_transactionHash, _events) => {
            enqueueSnackbar(
              `Amount Claimed, transaction hash: ${_transactionHash}`,
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
      positionId
    ]);
  }
  