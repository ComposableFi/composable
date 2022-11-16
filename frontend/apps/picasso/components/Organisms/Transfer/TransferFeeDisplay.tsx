import { humanBalance } from "shared";
import { FeeDisplay } from "@/components";
import { useEffect } from "react";
import { useStore } from "@/stores/root";
import { useExecutor } from "substrate-react";
import { useTransfer } from "@/defi/polkadot/hooks";

import { getDestChainFee } from "@/defi/polkadot/pallets/Transfer";
import { getPaymentAsset } from "@/defi/polkadot/pallets/AssetTxPayment";
import { Stack } from "@mui/material";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import { useAllParachainProviders } from "@/defi/polkadot/context/hooks";
import { subscribeTransactionFee } from "@/stores/defi/polkadot/transfers/subscribers";

export const TransferFeeDisplay = () => {
  const executor = useExecutor();
  const tokens = useStore(({ substrateTokens }) => substrateTokens.tokens);
  const setFeeToken = useStore((state) => state.transfers.setFeeToken);
  const feeToken = useStore((state) => state.transfers.feeToken);

  const { from, to, account, fromProvider } = useTransfer();
  const fee = useStore((state) => state.transfers.fee);
  const selectedToken = useStore((state) => state.transfers.selectedToken);
  const destFee = getDestChainFee(from, to, tokens, selectedToken);
  const allProviders = useAllParachainProviders();

  useEffect(() => {
    if (executor && account) {
      const unsub = subscribeTransactionFee(
        allProviders,
        account.address,
        executor
      );

      return () => {
        unsub.then((call) => call());
      };
    }
  }, [executor, allProviders, account]);

  useEffect(() => {
    if (fromProvider.parachainApi && account) {
      getPaymentAsset({
        api: fromProvider.parachainApi,
        walletAddress: account.address,
        network: from,
        tokens,
      }).then((token: TokenMetadata) => {
        setFeeToken(token.id);
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [fromProvider.parachainApi, account?.address, from]);

  return (
    <Stack direction="column" gap={4}>
      <FeeDisplay
        label="Fee"
        feeText={`${fee.partialFee.toFormat()} ${tokens[feeToken].symbol}`}
        TooltipProps={{
          title:
            "Fees(gas) for processing the given transaction. The amount can vary depending on transaction details and network conditions.",
        }}
      />
      {destFee.fee !== null && destFee.token !== null ? (
        <FeeDisplay
          label="Destination chain fee"
          feeText={`${destFee.fee.toFormat()} ${destFee.token.symbol}`}
          TooltipProps={{
            title:
              "Transaction fee on the destination chain. This fee is approximate and might change due to network conditions.",
          }}
        />
      ) : null}
    </Stack>
  );
};
