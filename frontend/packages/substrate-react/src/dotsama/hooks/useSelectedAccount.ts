import React from "react";
import { DotsamaContext } from "../DotSamaContext";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { ParachainId } from "shared";

export const useSelectedAccount = (
  parachainId: ParachainId
): InjectedAccountWithMeta | undefined => {
  const { selectedAccount, connectedAccounts } =
    React.useContext(DotsamaContext);

  return selectedAccount !== -1
    ? connectedAccounts[parachainId][selectedAccount]
    : undefined;
};
