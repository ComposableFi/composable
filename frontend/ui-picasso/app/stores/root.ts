import create from "zustand";
import { devtools } from "zustand/middleware";
import { createUISlice } from "./ui/ui";
import {
  createBondsSlice,
  createCrowdloanRewardsSlice,
  createMetamaskSlice,
  createOracleSlice,
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
import { AllSlices } from "./types";

export const useStore = create<AllSlices>()(
  devtools(
    immer((set, get) => ({
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
      ...createBondsSlice(set, get),
      ...createOracleSlice(set, get),
    }))
  )
);
