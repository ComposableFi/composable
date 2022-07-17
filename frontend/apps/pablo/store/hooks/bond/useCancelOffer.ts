import { APP_NAME } from "@/defi/polkadot/constants";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useSnackbar } from "notistack";
import { useCallback } from "react";
import {
  getSigner,
  useExecutor,
  useParachainApi,
  useSelectedAccount,
} from "substrate-react";

export function useCancelOffer() {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { enqueueSnackbar } = useSnackbar();

  const executor = useExecutor();

  const cancel = useCallback(
    async (offerId: number) => {
      if (!parachainApi || !selectedAccount || !executor) return null;
      const signer = await getSigner(APP_NAME, selectedAccount.address);

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
    [parachainApi, executor, selectedAccount, enqueueSnackbar]
  );

  return cancel;
}
