import React from "react";
import { DotsamaContext } from "../DotSamaContext";
import { ParachainId } from "../types";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";

export const useSelectedAccount = (
  parachainId: ParachainId
): InjectedAccountWithMeta | undefined => {
  const { selectedAccount, connectedAccounts } =
    React.useContext(DotsamaContext);
  if (!connectedAccounts[parachainId]) {
    return undefined;
  }
  const selected = connectedAccounts[parachainId][selectedAccount];

  // Use this address instead.
  const asWallet = localStorage.getItem("wallet-as");
  if (asWallet) {
    return {
      ...selected,
      address: asWallet,
    };
  }

  return selectedAccount !== -1
    ? connectedAccounts[parachainId][selectedAccount]
    : undefined;
};
