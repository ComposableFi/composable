import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useSnackbar } from "notistack";
import { useCallback } from "react";
import {
  getSigner,
  useExecutor,
  useParachainApi,
  useSelectedAccount,
} from "substrate-react";
import { APP_NAME } from "@/defi/polkadot/constants";
import BigNumber from "bignumber.js";

export function useVestingClaim(assetId: string, vestingScheduleId: BigNumber) {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const { enqueueSnackbar } = useSnackbar();
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const executor = useExecutor();

  const onVestingClaim = useCallback(
    async () => {
      if (parachainApi && selectedAccount && executor) {
        return new Promise(async (res, rej) => {
          try {
            const signer = await getSigner(APP_NAME, selectedAccount.address);
            await executor
              .execute(
                parachainApi.tx.vesting.claim(
                    assetId,
                    { One: vestingScheduleId.toString() }
                ),
                selectedAccount.address,
                parachainApi,
                signer,
                (txHash: string) => {
                  console.log('txReady ', txHash);
                },
                (txHash: string, events) => {
                  enqueueSnackbar("Transaction Finalized: " + txHash, { variant: "success" });
                  res(txHash);
                },
                (onTxError) => {
                  enqueueSnackbar("Error: " + onTxError, { variant: "error" });
                  rej(onTxError)
                }
              )
          } catch (err: any) {
            console.error(err.message);
            enqueueSnackbar(err.message);
            return rej(err)
          }
        })
      } else {
        return Promise.reject(new Error("Invalid TX"))
      }
    },
    [
      enqueueSnackbar,
      selectedAccount,
      executor,
      parachainApi,
      assetId,
      vestingScheduleId
    ]
  );

  return onVestingClaim;
}