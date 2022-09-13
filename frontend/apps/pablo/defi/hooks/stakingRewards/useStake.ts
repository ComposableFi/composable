import {
    getSigner,
    useExecutor,
    useParachainApi,
    useSelectedAccount,
  } from "substrate-react";
  import { DEFAULT_NETWORK_ID, toChainUnits } from "@/defi/utils";
  import _ from "lodash";
  import { useCallback } from "react";
  import BigNumber from "bignumber.js";
  import { useSnackbar } from "notistack";
  import { APP_NAME } from "@/defi/polkadot/constants";
  
  export type StakeProps = {
    poolId: BigNumber | undefined; // pool Id
    amount: BigNumber; // amount to stake
    durationPreset: BigNumber | undefined; // duration in seconds
  };
  
  export function useStake({ poolId, amount, durationPreset }: StakeProps) {
    const { enqueueSnackbar } = useSnackbar();
    const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
    const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
    const executor = useExecutor();
  
    return useCallback(async () => {
      if (!parachainApi || !selectedAccount || !executor || !poolId || !durationPreset) {
          throw new Error('Invalid stake pool.');
      }
  
      try {
        const signer = await getSigner(APP_NAME, selectedAccount.address);
        await executor.execute(
          parachainApi.tx.stakingRewards.stake(
            parachainApi.createType("u128", poolId.toString()),
            parachainApi.createType("u128", toChainUnits(amount).toString()),
            parachainApi.createType("u64", durationPreset.toString())
          ),
          selectedAccount.address,
          parachainApi,
          signer,
          (_transactionHash) => {
            console.log("Tx Ready: ", _transactionHash);
          },
          (_transactionHash, _events) => {
            enqueueSnackbar(
              `Amount staked: ${amount.toString()}, transaction hash: ${_transactionHash}`,
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
      amount,
      durationPreset,
      poolId,
    ]);
  }
  