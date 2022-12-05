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

export function usePurchaseBond(offerId: BigNumber, amount: BigNumber) {
  const signer = useSigner();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const { enqueueSnackbar } = useSnackbar();
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const executor = useExecutor();

  const purchaseBond = useCallback(
    async () => {
      if (parachainApi && signer !== undefined && selectedAccount && executor) {
        return new Promise(async (res, rej) => {
          try {
            await executor
              .execute(
                parachainApi.tx.bondedFinance.bond(offerId.toNumber(), amount.toString(), false),
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
            enqueueSnackbar(err.message, { variant: "error" });
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
      signer,
      parachainApi,
      offerId,
      amount
    ]
  );

  return purchaseBond;
}