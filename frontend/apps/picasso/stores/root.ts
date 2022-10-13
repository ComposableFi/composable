import create from "zustand";
import { createUISlice } from "./ui/ui";
import {
  createBondsSlice,
  createMetamaskSlice,
  createOracleSlice,
  createPolkadotSlice,
  createStakingRewardsSlice,
  createStatsApolloSlice,
  createStatsOverviewSlice,
  createStatsTelemetrySlice,
  createSubstrateBalancesSlice,
  createTransfersSlice,
} from "./defi";

import { AllSlices } from "./types";
import { immer } from "zustand/middleware/immer";
import { devtools, subscribeWithSelector } from "zustand/middleware";

export const useStore = create<AllSlices>()(
  subscribeWithSelector(
    immer(
      devtools((...a) => ({
        ...createUISlice(...a),
        ...createTransfersSlice(...a),
        ...createPolkadotSlice(...a),
        ...createMetamaskSlice(...a),
        ...createStatsApolloSlice(...a),
        ...createStatsOverviewSlice(...a),
        ...createStatsTelemetrySlice(...a),
        ...createSubstrateBalancesSlice(...a),
        ...createBondsSlice(...a),
        ...createOracleSlice(...a),
        ...createStakingRewardsSlice(...a),
      }))
    )
  )
);
