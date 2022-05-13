import { configureStore } from "@reduxjs/toolkit";
import uiReducer from "./ui/uiSlice";
import metamaskReducer from "./defi/metamask";
import substrateBalancesReducer from "./defi/polkadot/balances/slice";
import crowdloanRewardsSlice from "./defi/polkadot/crowdloanRewards/slice";
import transfersReducer from "./defi/transfers";
import polkadotReducer from "./defi/polkadot";
import stakingReducer from "./defi/staking";

export const store = configureStore({
  reducer: {
    ui: uiReducer,
    metamask: metamaskReducer,
    substrateBalances: substrateBalancesReducer,
    crowdloanRewards: crowdloanRewardsSlice,
    transfers: transfersReducer,
    polkadot: polkadotReducer,
    staking: stakingReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
