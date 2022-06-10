import create, { GetState, State } from "zustand";
import { devtools, NamedSet } from "zustand/middleware";
import { createUISlice } from "./ui/ui";
import {
  createCrowdloanRewardsSlice,
  createMetamaskSlice,
  createPolkadotSlice,
  createStakingSlice,
  createStatsApolloSlice,
  createStatsOverviewSlice,
  createStatsTelemetrySlice,
  createStatsTreasurySlice,
  createSubstrateBalancesSlice,
  createTransfersSlice,
} from "./defi";

import immer from "./middlewares/immer";
import { AppState, CustomStateCreator } from "./types";

export const createStore = <TState extends State>(
  storeCreator: CustomStateCreator<TState>
) => {
  return create(devtools(immer(storeCreator)));
};

export const useStore = createStore<AppState>(
  (set: NamedSet<any>, get: GetState<any>) => ({
    ...createUISlice(set, get),
    ...createTransfersSlice(set, get),
    ...createPolkadotSlice(set, get),
    ...createMetamaskSlice(set, get),
    ...createStakingSlice(set, get),
    ...createStatsApolloSlice(set, get),
    ...createStatsOverviewSlice(set, get),
    ...createStatsTelemetrySlice(set, get),
    ...createStatsTreasurySlice(set, get),
    ...createSubstrateBalancesSlice(set, get),
    ...createCrowdloanRewardsSlice(set, get),
  })
);
