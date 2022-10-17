import { StateCreator } from "zustand";

import {
  MetamaskSlice,
  OracleSlice,
  PolkadotSlice,
  StakingRewardsSlice,
  StatsApolloSlice,
  StatsOverviewSlice,
  StatsTelemetrySlice,
  SubstrateBalancesSlice,
  TransfersSlice,
} from "./defi";
import { UISlice } from "./ui/ui";
import { BondsSlice } from "@/stores/defi/polkadot/bonds/slice";

export type StoreSlice<T> = StateCreator<
  AllSlices,
  [
    ["zustand/subscribeWithSelector", never],
    ["zustand/immer", never],
    ["zustand/devtools", never]
  ],
  [],
  T
>;
export type AllSlices = PolkadotSlice &
  UISlice &
  MetamaskSlice &
  TransfersSlice &
  StatsApolloSlice &
  StatsOverviewSlice &
  StatsTelemetrySlice &
  SubstrateBalancesSlice &
  BondsSlice &
  StakingRewardsSlice &
  OracleSlice;
