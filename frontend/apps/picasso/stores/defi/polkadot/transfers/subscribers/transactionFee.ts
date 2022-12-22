import { AllProviders } from "@/defi/polkadot/context/hooks";
import { Executor, getSigner } from "substrate-react";
import { useStore } from "@/stores/root";
import { APP_NAME } from "@/defi/polkadot/constants";
import { fromChainIdUnit, SubstrateNetworkId, unwrapNumberOrHex } from "shared";
import BigNumber from "bignumber.js";
import { AssetRatio } from "@/defi/polkadot/pallets/Assets";
import { pipe } from "fp-ts/function";
import { either, option } from "fp-ts";

function getFeeInToken(
  partialFee: BigNumber,
  network: SubstrateNetworkId,
  ratio: AssetRatio | null
): BigNumber {
  return pipe(
    ratio,
    either.fromNullable(partialFee),
    either.fold(
      (partialFee) =>
        network === "picasso" ? option.none : option.some(partialFee),
      (r) => option.some(partialFee.multipliedBy(r.n).div(r.d))
    ),
    option.fold(
      () => new BigNumber(0),
      (fee) => fee
    )
  );
}

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
      selectedToken:
        store.substrateTokens.tokens[store.transfers.feeItem],
      from: store.transfers.networks.from,
    }),
    async ({
      selectedRecipient,
      sourceChain,
      isLoaded,
      transferExtrinsic,
      from,
      selectedToken,
    }) => {
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
        const partialFee = fromChainIdUnit(
          unwrapNumberOrHex(info.partialFee.toString())
        );
        // Either network is picasso or not
        // if network is not picasso
        // partialFee is enough
        // if network is picasso,
        // then we need to check if feeToken is set (BYOG)
        // pass the fee token to function

        useStore.getState().transfers.updateFee({
          class: info.class.toString(),
          partialFee: getFeeInToken(
            partialFee,
            from,
            selectedToken.ratio[from]
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
        a.amount.eq(b.amount) &&
        a.selectedToken === b.selectedToken &&
        a.from === b.from,
    }
  );
};
