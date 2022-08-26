import React from "react";
import { DotsamaContext } from "../DotSamaContext";
import { ParachainId, ConntectedAccount } from "../types";

export const useSelectedAccount = (
  parachainId: ParachainId
): ConntectedAccount | undefined => {
  const { selectedAccount, parachainProviders } =
    React.useContext(DotsamaContext);
  const { accounts } = parachainProviders[parachainId];
  return selectedAccount !== -1 ? accounts[selectedAccount] : undefined;
};
