import { ChainApi, useDotSamaContext } from "substrate-react";
import { useMemo } from "react";
import { SubstrateNetworkId } from "shared";

export const useAllParachainProviders = () => {
  const { parachainProviders, relaychainProviders } = useDotSamaContext();
  return useMemo(
    (): Record<SubstrateNetworkId, ChainApi> => ({
      ...parachainProviders,
      ...relaychainProviders,
    }),
    [parachainProviders, relaychainProviders]
  );
};
