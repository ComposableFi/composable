import { createSlice } from "@reduxjs/toolkit";

interface UIState {
  isMetamaskModalOpen: boolean;
  isPolkadotModalOpen: boolean;
  isConfirmingKSM: boolean;
  isConfirmingStablecoin: boolean;
  hasTriedEagerConnect: boolean;
  isClaimingKSM: boolean;
  isClaimingStablecoin: boolean;
  isClaimedKSM: boolean;
  isClaimedStablecoin: boolean;
}

const initialState: UIState = {
  isMetamaskModalOpen: false,
  isPolkadotModalOpen: false,
  isConfirmingKSM: false,
  isConfirmingStablecoin: false,
  hasTriedEagerConnect: false,
  isClaimingStablecoin: false,
  isClaimingKSM: false,
  isClaimedKSM: false,
  isClaimedStablecoin: false,
};

export const uiSlice = createSlice({
  name: "UI",
  initialState,
  reducers: {
    openMetamaskModal: (state) => {
      state.isMetamaskModalOpen = true;
    },
    closeMetamaskModal: (state) => {
      state.isMetamaskModalOpen = false;
    },
    openPolkadotModal: (state) => {
      state.isPolkadotModalOpen = true;
    },
    closePolkadotModal: (state) => {
      state.isPolkadotModalOpen = false;
    },
    openKSMClaimModal: (state) => {
      state.isClaimingKSM = true;
    },
    closeKSMClaimModal: (state) => {
      state.isClaimingKSM = false;
    },
    openStablecoinClaimModal: (state) => {
      state.isConfirmingStablecoin = true;
    },
    closeStablecoinClaimModal: (state) => {
      state.isConfirmingStablecoin = false;
    },
    setHasTriedEagerConnect: (state) => {
      state.hasTriedEagerConnect = true;
    },
    setIsClaimedKSM: (state, action) => {
      state.isClaimedKSM = action.payload;
    },
    setIsClaimedStablecoin: (state, action) => {
      state.isClaimedStablecoin = action.payload;
    },
  },
});

export const {
  openMetamaskModal,
  closeMetamaskModal,
  openPolkadotModal,
  closePolkadotModal,
  openKSMClaimModal,
  closeKSMClaimModal,
  openStablecoinClaimModal,
  closeStablecoinClaimModal,
  setHasTriedEagerConnect,
  setIsClaimedKSM,
  setIsClaimedStablecoin,
} = uiSlice.actions;

export default uiSlice.reducer;
