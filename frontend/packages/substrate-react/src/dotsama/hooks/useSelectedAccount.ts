import React from "react";
import { DotsamaContext } from "../DotSamaContext";
import { ParachainId } from "../types";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";

export const useSelectedAccount = (
  parachainId: ParachainId
): InjectedAccountWithMeta | undefined => {
  const { selectedAccount, connectedAccounts } =
    React.useContext(DotsamaContext);

  return selectedAccount !== -1 ? connectedAccounts[parachainId][selectedAccount] : undefined;
};
