import { GetState, State, StoreApi } from "zustand";
import { NamedSet } from "zustand/middleware";

import {
  CrowdloanRewardsSlice,
  MetamaskSlice,
  OracleSlice,
  PolkadotSlice,
  StakingSlice,
  StatsApolloSlice,
  StatsOverviewSlice,
  StatsTelemetrySlice,
  StatsTreasurySlice,
  SubstrateBalancesSlice,
  TransfersSlice,
} from "./defi";
import { UISlice } from "./ui/ui";
import { BondsSlice } from "@/stores/defi/polkadot/bonds/slice";

export type StoreSlice<T extends object> = (
  set: NamedSet<T>,
  get: GetState<T>
) => T;

export type CustomStateCreator<
  T extends State,
  CustomSetState = NamedSet<T>,
  CustomGetState = GetState<T>,
  CustomStoreApi extends StoreApi<T> = StoreApi<T>
> = (set: CustomSetState, get: CustomGetState, api: CustomStoreApi) => T;

export type AllSlices = PolkadotSlice &
  UISlice &
  MetamaskSlice &
  StakingSlice &
  TransfersSlice &
  StatsApolloSlice &
  StatsOverviewSlice &
  StatsTelemetrySlice &
  StatsTreasurySlice &
  SubstrateBalancesSlice &
  CrowdloanRewardsSlice &
  BondsSlice &
  OracleSlice;
