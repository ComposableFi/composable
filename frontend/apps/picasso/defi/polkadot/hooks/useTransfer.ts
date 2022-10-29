import { useAllParachainProviders } from "@/defi/polkadot/context/hooks";
import { useSelectedAccount } from "@/defi/polkadot/hooks/index";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { getAmountToTransfer } from "@/defi/polkadot/pallets/Transfer";
import {
  TransferHandlerArgs,
  transferKaruraPicasso,
  transferKusamaPicasso,
  transferPicassoKarura,
  transferPicassoKusama,
} from "@/defi/polkadot/pallets/xcmp";
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

  const {
    keepAlive,
    fee: { weight },
    tokenId,
    hasFeeItem,
    feeItem,
    amount,
    updateAmount: setAmount,
  } = useStore(({ transfers }) => transfers);

  const existentialDeposit = useStore(
    ({ substrateBalances }) =>
      substrateBalances.balances[from][SUBSTRATE_NETWORKS[from].tokenId]
        .existentialDeposit
  );
  const account = useSelectedAccount();
  const providers = useAllParachainProviders();
  const executor = useExecutor();

  const tokens = useStore(({ substrateTokens }) => substrateTokens.tokens);

  const transferToken = tokens[tokenId];

  const getBalance = useStore(
    (state) => state.transfers.getTransferTokenBalance
  );

  const prepareAndCall = async (
    transferHandler: (args: TransferHandlerArgs) => Promise<void>
  ) => {
    const api = providers[from].parachainApi;

    if (
      !signer ||
      !api ||
      !executor ||
      !account ||
      (hasFeeItem && feeItem.length === 0)
    ) {
      console.error("No API or Executor or account", {
        api,
        executor,
        account,
      });
      return;
    }
    const feeToken = tokens[feeItem]
    const TARGET_ACCOUNT_ADDRESS = selectedRecipient.length
      ? selectedRecipient
      : account.address;

    const TARGET_PARACHAIN_ID = SUBSTRATE_NETWORKS[to].parachainId;
    // Set amount to transfer
    const amountToTransfer = getAmountToTransfer({
      balance: getBalance(),
      amount,
      existentialDeposit,
      keepAlive,
      api,
      targetChain: to,
      sourceChain: from,
      tokens,
    });

    const transferHandlerArgs: TransferHandlerArgs = {
      api,
      targetChain: TARGET_PARACHAIN_ID,
      targetAccount: TARGET_ACCOUNT_ADDRESS,
      amount: amountToTransfer,
      enqueueSnackbar,
      executor,
      signer,
      weight,
      feeToken,
      transferToken: tokens[tokenId],
      signerAddress: account.address,
    };

    try {
      await transferHandler(transferHandlerArgs);
    } catch (err) {
      console.error(err);
    } finally {
      // clear amount after
      setAmount(new BigNumber(0));
    }
  };

  const transfer = async () => {
    let networkSpecificHandler = (_args: TransferHandlerArgs) =>
      Promise.resolve();
    switch (`${from}-${to}`) {
      case "kusama-picasso":
        networkSpecificHandler = transferKusamaPicasso;
        break;
      case "picasso-kusama":
        networkSpecificHandler = transferPicassoKusama;
        break;
      case "karura-picasso":
        networkSpecificHandler = transferKaruraPicasso;
        break;
      case "picasso-karura":
        networkSpecificHandler = transferPicassoKarura;
        break;
    }

    return prepareAndCall(networkSpecificHandler);
  };

  return {
    transfer,
    amount,
    from,
    to,
    balance: getBalance(),
    account,
    fromProvider,
    toProvider,
    transferToken,
  };
};
