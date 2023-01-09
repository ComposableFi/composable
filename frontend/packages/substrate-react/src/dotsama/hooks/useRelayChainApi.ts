import React from "react";
import { DotsamaContext } from "../DotSamaContext";
import { RelaychainId } from "shared";

export const useRelayChainApi = (relaychainId: RelaychainId) => {
  const { relaychainProviders } = React.useContext(DotsamaContext);
  return relaychainProviders[relaychainId];
};
