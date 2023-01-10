import { ChainApi, Executor } from "substrate-react";
import { useStore } from "@/stores/root";
import { fromChainIdUnit, SubstrateNetworkId, unwrapNumberOrHex } from "shared";
import BigNumber from "bignumber.js";
import { AssetRatio } from "@/defi/polkadot/pallets/Assets";
import { pipe } from "fp-ts/function";
import { option } from "fp-ts";
import { Signer } from "@polkadot/api/types";

function getFeeInToken(
  partialFee: BigNumber,
  network: SubstrateNetworkId,
  ratio: AssetRatio | null
): BigNumber {
  return pipe(
    ratio,
    option.fromNullable,
    option.map((r) => partialFee.multipliedBy(r.n).div(r.d)),
    option.fold(
      () => partialFee,
      (fee) => fee
    )
  );
}

export const subscribeTransactionFee = async (
  allProviders: { [chain in SubstrateNetworkId]: ChainApi },
  walletAddress: string,
  executor: Executor,
  signer: Signer
) => {
  return useStore.subscribe(
    (store) => ({
      isLoaded: store.substrateTokens.isLoaded,
      selectedRecipient: store.transfers.recipients.selected,
      sourceChain: store.transfers.networks.from,
      transferExtrinsic: store.transfers.transferExtrinsic,
      amount: store.transfers.amount,
      selectedToken: store.substrateTokens.tokens[store.transfers.feeItem],
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

      try {
        const info = await executor.paymentInfo(call, walletAddress, signer);
        const partialFee = new BigNumber(info.partialFee.toString());

        useStore.getState().transfers.updateFee({
          class: info.class.toString(),
          partialFee: fromChainIdUnit(
            getFeeInToken(partialFee, from, selectedToken.ratio[from]),
            selectedToken.decimals[from] ?? 12
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
