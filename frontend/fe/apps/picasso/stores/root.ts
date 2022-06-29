import { configureStore } from "@reduxjs/toolkit";
import uiReducer from "./ui/uiSlice";
import metamaskReducer from "./defi/metamask";
import substrateBalancesReducer from "./defi/polkadot/balances/slice";
import crowdloanRewardsSlice from "./defi/polkadot/crowdloanRewards/slice";
import transfersReducer from "./defi/transfers";
import polkadotReducer from "./defi/polkadot";
import stakingReducer from "./defi/staking";
import statsOverviewReducer from "./defi/stats/overview";
import statsTelemetryReducer from "./defi/stats/telemetry";
import statsTreasuryReducer from "./defi/stats/treasury";
import statsApolloReducer from "./defi/stats/apollo";
import bondsReducer from "./defi/polkadot/bonds/slice";
import oracleReducer from "./defi/polkadot/oracle/slice";

export const store = configureStore({
  reducer: {
    ui: uiReducer,
    metamask: metamaskReducer,
    substrateBalances: substrateBalancesReducer,
    crowdloanRewards: crowdloanRewardsSlice,
    transfers: transfersReducer,
    polkadot: polkadotReducer,
    staking: stakingReducer,
    bonding: bondsReducer,
    statsOverview: statsOverviewReducer,
    statsTelemetry: statsTelemetryReducer,
    statsTreasury: statsTreasuryReducer,
    statsApollo: statsApolloReducer,
    oracle: oracleReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
