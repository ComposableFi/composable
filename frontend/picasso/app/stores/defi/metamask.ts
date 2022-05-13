import { createSlice } from "@reduxjs/toolkit";

export type Account = {
  address: string;
};
interface MetamaskState {
  connected: boolean;
  connecting: boolean;
  eligible: boolean;
  account: Account;
  availableToClaim: number;
}

const initialState: MetamaskState = {
  connected: false,
  connecting: false,
  account: {
    address: "0x729e86ed5614348d66996f0E23f28012eaCb0D17",
  },
  eligible: true,
  availableToClaim: 122,
};

export const metamaskSlice = createSlice({
  name: "Metamask",
  initialState,
  reducers: {
    connectMetamaskWallet: (state) => {
      state.connected = true;
    },
    waitOnMetamaskWallet: (state) => {
      state.connecting = true;
    },
    disconnectMetamaskWallet: (state) => {
      state.connected = false;
    },
    setAvailableToClaim: (state, action) => {
      state.availableToClaim = action.payload;
    },
  },
});

export const {
  connectMetamaskWallet,
  disconnectMetamaskWallet,
  waitOnMetamaskWallet,
  setAvailableToClaim,
} = metamaskSlice.actions;

export default metamaskSlice.reducer;
