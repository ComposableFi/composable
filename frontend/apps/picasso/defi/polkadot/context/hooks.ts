import { useDotSamaContext } from "substrate-react";

export const useAllParachainProviders = () => {
  const { parachainProviders, relaychainProviders } = useDotSamaContext();
  return {
    ...parachainProviders,
    ...relaychainProviders
  };
};
