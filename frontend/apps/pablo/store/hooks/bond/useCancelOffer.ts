import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useSnackbar } from "notistack";
import { useCallback } from "react";
import {
  useExecutor,
  useParachainApi,
  useSelectedAccount,
  useSigner,
} from "substrate-react";

export function useCancelOffer() {
  const signer = useSigner();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { enqueueSnackbar } = useSnackbar();

  const executor = useExecutor();

  const cancel = useCallback(
    async (offerId: number) => {
      if (!parachainApi || !signer || !selectedAccount || !executor)
        return null;

      try {
        await executor
          .execute(
            parachainApi.tx.bondedFinance.cancel(offerId),
            selectedAccount.address,
            parachainApi,
            signer,
            (txHash: string) => {
              enqueueSnackbar("Initiating Transaction on " + txHash);
            },
            (txHash: string, events) => {
              enqueueSnackbar("Transaction Finalized on " + txHash);
            }
          )
          .catch((err) => {
            enqueueSnackbar(err.message);
          });
        return true;
      } catch (err: any) {
        enqueueSnackbar(err.message);
        return null;
      }
    },
    [parachainApi, signer, executor, selectedAccount, enqueueSnackbar]
  );

  return cancel;
}
