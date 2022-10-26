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
import { AssetId } from "@/defi/polkadot/types";
import { useStore } from "@/stores/root";
import BigNumber from "bignumber.js";
import { useSnackbar } from "notistack";
import { useExecutor } from "substrate-react";

export const useTransfer = () => {
  const allProviders = useAllParachainProviders();
  const from = useStore((state) => state.transfers.networks.from);
  const fromProvider = allProviders[from];
  const to = useStore((state) => state.transfers.networks.to);
  const toProvider = allProviders[to];
  const { enqueueSnackbar } = useSnackbar();
  const selectedRecipient = useStore(
    (state) => state.transfers.recipients.selected
  );
  const { hasFeeItem, feeItem } = useStore(({ transfers }) => transfers);
  const weight = useStore((state) => state.transfers.fee.weight);
  const keepAlive = useStore((state) => state.transfers.keepAlive);
  const existentialDeposit = useStore(
    ({ substrateBalances }) =>
      substrateBalances.assets[from].native.existentialDeposit
  );
  const selectedToken = useStore(state => state.transfers.selectedToken);
  const amount = useStore((state) => state.transfers.amount);
  const setAmount = useStore((state) => state.transfers.updateAmount);
  const account = useSelectedAccount();
  const providers = useAllParachainProviders();
  const executor = useExecutor();
  const assets = useStore(
    ({ substrateBalances }) => substrateBalances.assets[from].assets
  );
  const getBalance = useStore(
    (state) => state.transfers.getTransferTokenBalance
  );

  const prepareAndCall = async (
    transferHandler: (args: TransferHandlerArgs) => Promise<void>
  ) => {
    const api = providers[from].parachainApi;

    if (!api || !executor || !account || (hasFeeItem && feeItem.length === 0)) {
      console.error("No API or Executor or account", {
        api,
        executor,
        account,
      });
      return;
    }
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
      tokenId: selectedToken
    });

    const feeItemId =
      hasFeeItem && feeItem.length > 0
        ? assets[feeItem as AssetId].meta.supportedNetwork[from]
        : null;

    const signerAddress = account.address;

    await transferHandler({
      api,
      targetChain: TARGET_PARACHAIN_ID,
      targetAccount: TARGET_ACCOUNT_ADDRESS,
      amount: amountToTransfer,
      executor,
      enqueueSnackbar,
      signerAddress,
      hasFeeItem,
      feeItemId,
      weight,
      token: selectedToken
    });

    // clear amount after
    setAmount(new BigNumber(0));
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
  };
};
