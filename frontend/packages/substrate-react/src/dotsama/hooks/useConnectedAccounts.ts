import React from "react";
import { DotsamaContext } from "../DotSamaContext";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { ParachainId, RelaychainId } from "shared";

export const useConnectedAccounts = (
  parachainId: ParachainId | RelaychainId
): InjectedAccountWithMeta[] => {
  const { connectedAccounts } = React.useContext(DotsamaContext);

  return connectedAccounts[parachainId] ? connectedAccounts[parachainId] : [];
};
