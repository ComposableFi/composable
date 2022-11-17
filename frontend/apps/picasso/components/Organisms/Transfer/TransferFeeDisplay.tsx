import { FeeDisplay } from "@/components";
import { TextExpander } from "@/components/Molecules/TextExpander";
import { useAllParachainProviders } from "@/defi/polkadot/context/hooks";
import { useTransfer } from "@/defi/polkadot/hooks";
import { getPaymentAsset } from "@/defi/polkadot/pallets/AssetTxPayment";

import { getDestChainFee } from "@/defi/polkadot/pallets/Transfer";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import { subscribeTransactionFee } from "@/stores/defi/polkadot/transfers/subscribers";
import { useStore } from "@/stores/root";
import { Stack, Typography } from "@mui/material";
import { useEffect } from "react";
import { humanBalance } from "shared";
import {
  DESTINATION_FEE_MULTIPLIER,
  FEE_MULTIPLIER,
} from "shared/defi/constants";
import { useExecutor } from "substrate-react";

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
        feeText={
          <TextExpander
            short={
              <Typography variant="body2">
                {humanBalance(fee.partialFee.multipliedBy(FEE_MULTIPLIER))}{" "}
                {tokens[feeToken].symbol}
              </Typography>
            }
            expanded={
              <Typography variant="body2">
                {fee.partialFee
                  .multipliedBy(FEE_MULTIPLIER)
                  .toFormat(tokens[feeToken].decimals[from] ?? 12)}{" "}
                {tokens[feeToken].symbol}
              </Typography>
            }
          />
        }
        TooltipProps={{
          title:
            "Fees(gas) for processing the given transaction. The amount can vary depending on transaction details and network conditions.",
        }}
      />
      {destFee && destFee?.fee !== null && destFee?.token !== null ? (
        <FeeDisplay
          label="Destination chain fee"
          feeText={
            <TextExpander
              short={
                <Typography variant="body2">
                  {humanBalance(
                    destFee.fee.multipliedBy(DESTINATION_FEE_MULTIPLIER)
                  )}{" "}
                  {destFee.token.symbol}
                </Typography>
              }
              expanded={
                <Typography variant="body2">
                  {destFee.fee.toFormat(destFee.token.decimals[to] ?? 12)}{" "}
                  {destFee.token.symbol}
                </Typography>
              }
            />
          }
          TooltipProps={{
            title:
              "Transaction fee on the destination chain. This fee is approximate and might change due to network conditions.",
          }}
        />
      ) : null}
    </Stack>
  );
};
