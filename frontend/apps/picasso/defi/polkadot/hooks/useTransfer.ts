import { useAllParachainProviders } from "@/defi/polkadot/context/hooks";
import { useSelectedAccount } from "@/defi/polkadot/hooks/index";
import { useStore } from "@/stores/root";
import { SnackbarKey, useSnackbar } from "notistack";
import { useExecutor, useSigner } from "substrate-react";
import BigNumber from "bignumber.js";
import { xcmPalletEventParser } from "@/defi/polkadot/pallets/XCM/utils";
import { subscanExtrinsicLink } from "shared";
import { useRef } from "react";

export const useTransfer = () => {
  const allProviders = useAllParachainProviders();
  const from = useStore((state) => state.transfers.networks.from);
  const fromProvider = allProviders[from];
  const to = useStore((state) => state.transfers.networks.to);
  const toProvider = allProviders[to];
  const signer = useSigner();
  const { enqueueSnackbar, closeSnackbar } = useSnackbar();
  const selectedRecipient = useStore(
    (state) => state.transfers.recipients.selected
  );
  const feeToken = useStore(({ transfers }) => transfers.feeToken);
  const amount = useStore((state) => state.transfers.amount);
  const setAmount = useStore((state) => state.transfers.updateAmount);
  const account = useSelectedAccount();
  const providers = useAllParachainProviders();
  const executor = useExecutor();

  const getBalance = useStore(
    (state) => state.transfers.getTransferTokenBalance
  );
  const makeTransferCall = useStore(
    (state) => state.transfers.makeTransferCall
  );

  const isDirty = useRef(false);

  const TARGET_ACCOUNT_ADDRESS = selectedRecipient.length
    ? selectedRecipient
    : account?.address;

  const transfer = async () => {
    const api = providers[from].parachainApi;

    if (!signer || !api || !executor || !account || feeToken.length === 0) {
      console.error("No API or Executor or account", {
        api,
        executor,
        account,
      });
      return;
    }

    const signerAddress = account.address;
    const call = makeTransferCall(api, TARGET_ACCOUNT_ADDRESS);
    if (!call) {
      console.error("Could not make transfer extrinsic");
      return;
    }
    try {
      let snackbarKey: SnackbarKey;
      await executor.execute(
        call,
        signerAddress,
        api,
        signer,
        (txHash) => {
          snackbarKey = enqueueSnackbar(
            "Executing transfer... just one moment, please.",
            {
              persist: true,
              description: "",
              variant: "info",
              isCloseable: true,
              url: subscanExtrinsicLink(from, txHash),
            }
          );
        },
        (txHash, records) => {
          if (api.events.xcmPallet || api.events.polkadotXcm) {
            xcmPalletEventParser(
              records,
              api,
              closeSnackbar,
              snackbarKey,
              enqueueSnackbar,
              txHash,
              from
            );
          } else {
            closeSnackbar(snackbarKey);
            enqueueSnackbar("Transfer is successful", {
              persist: true,
              description: "",
              variant: "success",
              isCloseable: true,
              url: subscanExtrinsicLink(from, txHash),
            });
          }

          setAmount(new BigNumber(0));
          isDirty.current = false;
        },
        (err) => {
          snackbarKey = enqueueSnackbar("Transfer failed", {
            persist: true,
            description: `Error: ${err}`,
            variant: "error",
            isCloseable: true,
          });
        }
      );
    } catch (e) {
      if (e instanceof Error) {
        enqueueSnackbar(e.toString(), {
          persist: true,
          description: "",
          variant: "error",
          isCloseable: true,
        });
        console.warn(e);
      }
    }
  };

  return {
    transfer,
    amount,
    from,
    to,
    balance: getBalance(),
    account,
    fromProvider,
    setAmount,
    toProvider,
    TARGET_ACCOUNT_ADDRESS,
    isDirty,
  };
};
