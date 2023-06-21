import {
  ParachainApi,
  RelaychainApi,
  useDotSamaContext
} from "substrate-react";
import { useMemo } from "react";

export type AllProviders = {
  kusama: RelaychainApi;
  polkadot: RelaychainApi;
  karura: ParachainApi;
  picasso: ParachainApi;
  statemine: ParachainApi;
};
export const useAllParachainProviders: () => AllProviders = () => {
  const { parachainProviders, relaychainProviders } = useDotSamaContext();
  return useMemo(
    () => ({
      ...parachainProviders,
      ...relaychainProviders
    }),
    [parachainProviders, relaychainProviders]
  );
};
