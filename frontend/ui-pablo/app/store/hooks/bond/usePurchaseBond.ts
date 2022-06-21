import { useSnackbar } from "notistack";
import { useCallback } from "react";
import {
  getSigner,
  useExecutor,
  useParachainApi,
  useSelectedAccount,
} from "substrate-react";
import { APP_NAME } from "../../../defi/polkadot/constants";
import { DEFAULT_NETWORK_ID } from "../../../updaters/constants";

export function usePurchaseBond() {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { enqueueSnackbar } = useSnackbar();

  const executor = useExecutor();

  const bond = useCallback(
    async (offerId: number, nbOfBonds: number) => {
      if (!parachainApi || !selectedAccount || !executor) return null;
      const signer = await getSigner(APP_NAME, selectedAccount.address);

      try {
        await executor
          .execute(
            parachainApi.tx.bondedFinance.bond(offerId, nbOfBonds, true),
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
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [parachainApi]
  );

  return bond;
}
