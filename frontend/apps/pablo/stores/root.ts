import { AnyAction, configureStore, EnhancedStore } from "@reduxjs/toolkit";
import uiReducer from "./ui/uiSlice";
import polkadotReducer from "./defi/polkadot";
import poolReducer from "./defi/pool";
import settingsReducer from "./defi/settings";

export const store: EnhancedStore<any, AnyAction, any[]> = configureStore({
  reducer: {
    ui: uiReducer,
    polkadot: polkadotReducer,
    pool: poolReducer,
    settings: settingsReducer,
  },
  middleware: [],
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
