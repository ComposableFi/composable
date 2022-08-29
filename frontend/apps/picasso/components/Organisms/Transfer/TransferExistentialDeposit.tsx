import { FeeDisplay } from "@/components";
import React from "react";
import { useStore } from "@/stores/root";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { useExistentialDeposit } from "@/components/Organisms/Transfer/hooks";

export const TransferExistentialDeposit = ({
  network,
}: {
  network: SubstrateNetworkId;
}) => {
  const { existentialDeposit, tokenId } = useExistentialDeposit();
  return (
    <FeeDisplay
      label="Existential Deposit"
      feeText={`${existentialDeposit.toString()} ${tokenId.toUpperCase()}`}
      TooltipProps={{
        title: `On the Polkadot network, an address is only active when it holds a minimum amount, currently set at 1 DOT (and 0.0000333333 KSM on the Kusama network). This minimum amount is called the Existential Deposit (ED).`,
      }}
    />
  );
};
