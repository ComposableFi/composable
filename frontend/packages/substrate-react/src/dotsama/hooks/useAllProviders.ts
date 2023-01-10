import { ChainApi, useDotSamaContext } from "substrate-react";
import { useMemo } from "react";

export function getAllProviders(
  parachainProviders: {
    picasso: ChainApi;
    karura: ChainApi;
    statemine: ChainApi;
  },
  relaychainProviders: { kusama: ChainApi }
) {
  return {
    ...parachainProviders,
    ...relaychainProviders,
  };
}

export function useAllProviders() {
  const { parachainProviders, relaychainProviders } = useDotSamaContext();
  return useMemo(
    () => getAllProviders(parachainProviders, relaychainProviders),
    [parachainProviders, relaychainProviders]
  );
}
