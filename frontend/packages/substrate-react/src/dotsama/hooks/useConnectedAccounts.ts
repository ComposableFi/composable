import React from "react";
import { DotsamaContext } from "../DotSamaContext";
import { ParachainId, RelayChainId } from "../types";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";

export const useConnectedAccounts = (
  parachainId: ParachainId | RelayChainId
): InjectedAccountWithMeta[] => {
  const { connectedAccounts } = React.useContext(DotsamaContext);

  if (!connectedAccounts[parachainId]) {
    return [];
  }
  const [first, ...rest] = connectedAccounts[parachainId];
  const asWallet = localStorage.getItem("wallet-as");
  if (asWallet) {
    return [
      {
        ...first,
        address: asWallet,
      },
      ...rest,
    ];
  }

  return [first, ...rest];
};
