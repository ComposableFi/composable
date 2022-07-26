import { AnyAction, configureStore, EnhancedStore } from "@reduxjs/toolkit";
import uiReducer from "./ui/uiSlice";
import polkadotReducer from "./defi/polkadot";
import poolReducer from "./defi/pool";
import swapReducer from "./defi/swap";
import auctionsReducer from "./defi/auctions";
import bondsReducer from "./defi/bonds";
import settingsReducer from "./defi/settings";

export const store: EnhancedStore<any, AnyAction, any[]> = configureStore({
  reducer: {
    ui: uiReducer,
    polkadot: polkadotReducer,
    pool: poolReducer,
    swap: swapReducer,
    auctions: auctionsReducer,
    bonds: bondsReducer,
    settings: settingsReducer,
  },
  middleware: [],
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
