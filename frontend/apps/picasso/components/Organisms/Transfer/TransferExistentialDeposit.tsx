import { FeeDisplay } from "@/components";
import { TextExpander } from "@/components/Molecules/TextExpander";
import React, { FC } from "react";
import { useStore } from "@/stores/root";
import { Chip, Typography } from "@mui/material";
import { humanBalance } from "shared";

/**
 * Existential deposit should be derived from, and in order:
 * Transfer asset's Existential deposit from the chain
 *
 * Examples:
 * From Picasso => Karura. Sending KSM
 * Fetch KSM existential deposit on picasso chain
 *
 * From Kusama => Picasso, sending KSM
 * Fetch KSM ed on Kusama chain
 *
 * From Karura => Picasso, sending KUSD,
 * Fetch KUSD ed on Karura chain
 */
export const TransferExistentialDeposit: FC = () => {
  const from = useStore((state) => state.transfers.networks.from);
  const selectedToken = useStore(
    (state) => state.substrateTokens.tokens[state.transfers.selectedToken]
  );
  const existentialDeposit = selectedToken.existentialDeposit[from];
  const decimals = selectedToken.decimals[from] ?? Number(0);

  return (
    <FeeDisplay
      label="Existential Deposit"
      feeText={
        decimals !== null && !!existentialDeposit ? (
          <TextExpander
            short={
              <Typography variant="body2">
                {humanBalance(existentialDeposit)} {selectedToken.symbol}
              </Typography>
            }
            expanded={
              <Typography variant="body2">
                {existentialDeposit.toFormat(
                  selectedToken.decimals[from] ?? 12
                )}{" "}
                {selectedToken.symbol}
              </Typography>
            }
          />
        ) : (
          <Chip
            variant="filled"
            color="error"
            label="Could not fetch Existential deposit"
          />
        )
      }
      TooltipProps={{
        title: `On the Polkadot network, an address is only active when it holds a minimum amount, currently set at 1 DOT (and 0.0000333333 KSM on the Kusama network). This minimum amount is called the Existential Deposit (ED) and prevents account removal.`,
      }}
    />
  );
};
