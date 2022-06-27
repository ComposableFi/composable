import { NamedSet } from "zustand/middleware";

import { StoreSlice } from "../types";

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

export interface UISlice {
  ui: UIState & {
    openMetamaskModal: () => void;
    closeMetamaskModal: () => void;
    openPolkadotModal: () => void;
    closePolkadotModal: () => void;
    openKSMClaimModal: () => void;
    closeKSMClaimModal: () => void;
    openStablecoinClaimModal: () => void;
    closeStablecoinClaimModal: () => void;
    setHasTriedEagerConnect: () => void;
    setIsClaimedKSM: (isClaimedKSM: boolean) => void;
    setIsClaimedStablecoin: (isClaimedStablecoin: boolean) => void;
  };
}

export const createUISlice: StoreSlice<UISlice> = (set: NamedSet<UISlice>) => ({
  ui: {
    ...initialState,

    openMetamaskModal: () => {
      set(function fetchClaimTotals(state) {
        state.ui.isMetamaskModalOpen = true;
        return state;
      });
    },
    closeMetamaskModal: () => {
      set(function fetchClaimTotals(state) {
        state.ui.isMetamaskModalOpen = false;
        return state;
      });
    },
    openPolkadotModal: () => {
      set(function fetchClaimTotals(state) {
        state.ui.isPolkadotModalOpen = true;
        return state;
      });
    },
    closePolkadotModal: () => {
      set(function fetchClaimTotals(state) {
        state.ui.isPolkadotModalOpen = false;
        return state;
      });
    },
    openKSMClaimModal: () => {
      set(function fetchClaimTotals(state) {
        state.ui.isClaimingKSM = true;
        return state;
      });
    },
    closeKSMClaimModal: () => {
      set(function fetchClaimTotals(state) {
        state.ui.isClaimingKSM = false;
        return state;
      });
    },
    openStablecoinClaimModal: () => {
      set(function fetchClaimTotals(state) {
        state.ui.isConfirmingStablecoin = true;
        return state;
      });
    },
    closeStablecoinClaimModal: () => {
      set(function fetchClaimTotals(state) {
        state.ui.isConfirmingStablecoin = false;
        return state;
      });
    },
    setHasTriedEagerConnect: () => {
      set(function fetchClaimTotals(state) {
        state.ui.hasTriedEagerConnect = true;
        return state;
      });
    },
    setIsClaimedKSM: (isClaimedKSM) => {
      set(function fetchClaimTotals(state) {
        state.ui.isClaimedKSM = isClaimedKSM;
        return state;
      });
    },
    setIsClaimedStablecoin: (isClaimedStablecoin) => {
      set(function fetchClaimTotals(state) {
        state.ui.isClaimedStablecoin = isClaimedStablecoin;
        return state;
      });
    },
  },
});
