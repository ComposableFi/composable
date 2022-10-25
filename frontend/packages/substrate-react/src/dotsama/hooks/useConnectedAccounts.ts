import React from "react";
import { DotsamaContext } from "../DotSamaContext";
import { ParachainId } from "../types";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";

export const useConnectedAccounts = (
  parachainId: ParachainId
): InjectedAccountWithMeta[] => {
  const { connectedAccounts } =
    React.useContext(DotsamaContext);

  return connectedAccounts[parachainId] ? connectedAccounts[parachainId] : [];
};
