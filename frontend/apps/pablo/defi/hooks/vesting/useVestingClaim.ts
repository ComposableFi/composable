import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useSnackbar } from "notistack";
import { useCallback } from "react";
import {
  useExecutor,
  useParachainApi,
  useSelectedAccount,
  useSigner,
} from "substrate-react";
import BigNumber from "bignumber.js";

export function useVestingClaim(assetId: string, vestingScheduleId: BigNumber) {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const { enqueueSnackbar } = useSnackbar();
  const signer = useSigner();
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const executor = useExecutor();

  return useCallback(
    async () => {
      if (parachainApi && signer && selectedAccount && executor && vestingScheduleId.gte(0)) {
        return new Promise(async (res, rej) => {
          try {
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
    [parachainApi, signer, selectedAccount, executor, vestingScheduleId, assetId, enqueueSnackbar]
  );
}