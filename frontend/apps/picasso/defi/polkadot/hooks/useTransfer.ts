import { useSnackbar } from "notistack";
import { useStore } from "@/stores/root";
import { useMemo } from "react";
import {
  getTransferToken,
  TransferHandlerArgs,
  transferKaruraPicasso,
  transferKusamaPicasso,
  transferPicassoKarura,
  transferPicassoKusama
} from "@/defi/polkadot/pallets/xcmp";
import { useSelectedAccount } from "@/defi/polkadot/hooks/index";
import { useAllParachainProviders } from "@/defi/polkadot/context/hooks";
import { useExecutor } from "substrate-react";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { getAmountToTransfer } from "@/defi/polkadot/pallets/Transfer";
import { AssetId } from "@/defi/polkadot/types";

export const useTransfer = () => {
  const allProviders = useAllParachainProviders();
  const from = useStore(state => state.transfers.networks.from);
  const fromProvider = allProviders[from];
  const to = useStore(state => state.transfers.networks.to);
  const toProvider = allProviders[to];

  const { enqueueSnackbar } = useSnackbar();

  const selectedRecipient = useStore(
    state => state.transfers.recipients.selected
  );

  const assets = useStore(
    ({ substrateBalances }) => substrateBalances.assets[from].assets
  );

  const { hasFeeItem, feeItem } = useStore(({ transfers }) => transfers);
  const weight = useStore(state => state.transfers.fee.weight);
  const native = useStore(
    ({ substrateBalances }) => substrateBalances.assets[from].native
  );
  const keepAlive = useStore(state => state.transfers.keepAlive);
  const existentialDeposit = useStore(
    ({ substrateBalances }) =>
      substrateBalances.assets[from].native.existentialDeposit
  );
  const isTokenNative = useMemo(() => {
    return assets[getTransferToken(from, to)].meta.supportedNetwork[from] === 1;
  }, [from, to, assets]);
  const amount = useStore(state => state.transfers.amount);
  const tokenId = useStore(state => state.transfers.tokenId);
  const balance = isTokenNative ? native.balance : assets[tokenId].balance;

  const account = useSelectedAccount();
  const providers = useAllParachainProviders();
  const executor = useExecutor();

  const prepareAndCall = async (
    transferHandler: (args: TransferHandlerArgs) => Promise<void>
  ) => {
    const api = providers[from].parachainApi;

    if (!api || !executor || !account || (hasFeeItem && feeItem.length === 0)) {
      console.error("No API or Executor or account", {
        api,
        executor,
        account
      });
      return;
    }
    const TARGET_ACCOUNT_ADDRESS = selectedRecipient.length
      ? selectedRecipient
      : account.address;

    const TARGET_PARACHAIN_ID = SUBSTRATE_NETWORKS[to].parachainId;
    // Set amount to transfer
    const amountToTransfer = getAmountToTransfer({
      balance,
      amount,
      existentialDeposit,
      keepAlive,
      api
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
      weight
    });
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
    balance,
    account,
    fromProvider,
    toProvider,
    isTokenNative,
    tokenId
  };
};
