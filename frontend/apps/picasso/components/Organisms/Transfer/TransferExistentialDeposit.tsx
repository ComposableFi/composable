import { FeeDisplay } from "@/components";
import React, { FC } from "react";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { useExistentialDeposit } from "@/defi/polkadot/hooks/useExistentialDeposit";
import { useStore } from "@/stores/root";

export const TransferExistentialDeposit: FC = () => {
  const from = useStore((state) => state.transfers.networks.from);
  const existentialDeposit = useStore(
    (state) => state.transfers.existentialDeposit
  );
  const getFeeToken = useStore((state) => state.transfers.getFeeToken);
  const feeToken = getFeeToken(from);

  return (
    <FeeDisplay
      label="Existential Deposit"
      feeText={`${existentialDeposit.toString()} ${feeToken.symbol.toUpperCase()}`}
      TooltipProps={{
        title: `On the Polkadot network, an address is only active when it holds a minimum amount, currently set at 1 DOT (and 0.0000333333 KSM on the Kusama network). This minimum amount is called the Existential Deposit (ED).`,
      }}
    />
  );
};
