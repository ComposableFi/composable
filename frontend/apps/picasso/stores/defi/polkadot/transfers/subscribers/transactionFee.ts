import { AllProviders } from "@/defi/polkadot/context/hooks";
import { Executor, getSigner } from "substrate-react";
import { useStore } from "@/stores/root";
import { APP_NAME } from "@/defi/polkadot/constants";
import { fromChainIdUnit, unwrapNumberOrHex } from "shared";
import BigNumber from "bignumber.js";

export const subscribeTransactionFee = async (
  allProviders: AllProviders,
  walletAddress: string,
  executor: Executor
) => {
  return useStore.subscribe(
    (store) => ({
      isLoaded: store.substrateTokens.isLoaded,
      selectedRecipient: store.transfers.recipients.selected,
      sourceChain: store.transfers.networks.from,
      transferExtrinsic: store.transfers.transferExtrinsic,
      amount: store.transfers.amount,
    }),
    async ({ selectedRecipient, sourceChain, isLoaded, transferExtrinsic }) => {
      if (!isLoaded || !transferExtrinsic) return;
      const recipient = selectedRecipient.length
        ? selectedRecipient
        : walletAddress;

      const api = allProviders[sourceChain].parachainApi;
      if (!api) return;

      const call = useStore
        .getState()
        .transfers.makeTransferCall(api, recipient);
      if (!call) return;

      const signer = await getSigner(APP_NAME, walletAddress);
      try {
        const info = await executor.paymentInfo(call, walletAddress, signer);
        useStore.getState().transfers.updateFee({
          class: info.class.toString(),
          partialFee: fromChainIdUnit(
            unwrapNumberOrHex(info.partialFee.toString())
          ),
          weight: unwrapNumberOrHex(info.weight.toString()),
        } as {
          class: string;
          partialFee: BigNumber;
          weight: BigNumber;
        });
      } catch (e) {
        useStore.getState().transfers.updateFee({
          class: "",
          partialFee: new BigNumber(0),
          weight: new BigNumber(0),
        } as {
          class: string;
          partialFee: BigNumber;
          weight: BigNumber;
        });
      }
    },
    {
      equalityFn: (a, b) =>
        a.selectedRecipient === b.selectedRecipient &&
        a.sourceChain === b.sourceChain &&
        a.isLoaded === b.isLoaded &&
        a.transferExtrinsic === b.transferExtrinsic &&
        a.amount.eq(b.amount),
    }
  );
};
