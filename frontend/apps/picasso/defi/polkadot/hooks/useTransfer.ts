import { useAllParachainProviders } from "@/defi/polkadot/context/hooks";
import { useSelectedAccount } from "@/defi/polkadot/hooks/index";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { useStore } from "@/stores/root";
import { useSnackbar } from "notistack";
import { useExecutor, useSigner } from "substrate-react";
import BigNumber from "bignumber.js";

export const useTransfer = () => {
  const allProviders = useAllParachainProviders();
  const from = useStore((state) => state.transfers.networks.from);
  const fromProvider = allProviders[from];
  const to = useStore((state) => state.transfers.networks.to);
  const toProvider = allProviders[to];
  const signer = useSigner();
  const { enqueueSnackbar } = useSnackbar();
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
      console.log(call);
      console.error("Could not make transfer extrinsic");
      return;
    }

    await executor.execute(
      call,
      signerAddress,
      api,
      signer,
      (txHash) => {
        enqueueSnackbar("Transfer executed", {
          persist: true,
          description: `Transaction hash: ${txHash}`,
          variant: "info",
          isCloseable: true,
          url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
        });
      },
      (txHash) => {
        enqueueSnackbar("Transfer executed successfully.", {
          persist: true,
          variant: "success",
          isCloseable: true,
          url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
        });
        setAmount(new BigNumber(0));
      },
      (err) => {
        enqueueSnackbar("Transfer failed", {
          persist: true,
          description: `Error: ${err}`,
          variant: "error",
          isCloseable: true,
        });
      }
    );
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
  };
};
