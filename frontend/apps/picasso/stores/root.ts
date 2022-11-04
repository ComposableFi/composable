import create from "zustand";
import { createUISlice } from "./ui/ui";
import {
  createTokensSlice,
  createBondsSlice,
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
        ...createTokensSlice(...a),
        ...createTransfersSlice(...a),
        ...createPolkadotSlice(...a),
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
