import create from "zustand";
import { createUISlice } from "./ui/ui";
import {
  createBondsSlice,
  createOracleSlice,
  createPolkadotSlice,
  createPoolsSlice,
  createStakingRewardsSlice,
  createStatsApolloSlice,
  createStatsTelemetrySlice,
  createSubstrateBalancesSlice,
  createTokensSlice,
  createTransfersSlice,
} from "./defi";

import { AllSlices } from "./types";
import { immer } from "zustand/middleware/immer";
import { devtools, subscribeWithSelector } from "zustand/middleware";
import { enableMapSet } from "immer";

enableMapSet();

export const useStore = create<AllSlices>()(
  subscribeWithSelector(
    immer(
      devtools((...a) => ({
        ...createUISlice(...a),
        ...createTokensSlice(...a),
        ...createTransfersSlice(...a),
        ...createPolkadotSlice(...a),
        ...createStatsApolloSlice(...a),
        ...createStatsTelemetrySlice(...a),
        ...createSubstrateBalancesSlice(...a),
        ...createBondsSlice(...a),
        ...createOracleSlice(...a),
        ...createStakingRewardsSlice(...a),
        ...createPoolsSlice(...a),
      }))
    )
  )
);
