import { NamedSet } from "zustand/middleware";

import { AllSlices, StoreSlice } from "../types";

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
      set(function fetchClaimTotals(state: AllSlices) {
        state.ui.isMetamaskModalOpen = true;
      });
    },
    closeMetamaskModal: () => {
      set(function fetchClaimTotals(state: AllSlices) {
        state.ui.isMetamaskModalOpen = false;
      });
    },
    openPolkadotModal: () => {
      set(function fetchClaimTotals(state: AllSlices) {
        state.ui.isPolkadotModalOpen = true;
      });
    },
    closePolkadotModal: () => {
      set(function fetchClaimTotals(state: AllSlices) {
        state.ui.isPolkadotModalOpen = false;
      });
    },
    openKSMClaimModal: () => {
      set(function fetchClaimTotals(state: AllSlices) {
        state.ui.isClaimingKSM = true;
      });
    },
    closeKSMClaimModal: () => {
      set(function fetchClaimTotals(state: AllSlices) {
        state.ui.isClaimingKSM = false;
      });
    },
    openStablecoinClaimModal: () => {
      set(function fetchClaimTotals(state: AllSlices) {
        state.ui.isConfirmingStablecoin = true;
      });
    },
    closeStablecoinClaimModal: () => {
      set(function fetchClaimTotals(state: AllSlices) {
        state.ui.isConfirmingStablecoin = false;
      });
    },
    setHasTriedEagerConnect: () => {
      set(function fetchClaimTotals(state: AllSlices) {
        state.ui.hasTriedEagerConnect = true;
      });
    },
    setIsClaimedKSM: (isClaimedKSM) => {
      set(function fetchClaimTotals(state: AllSlices) {
        state.ui.isClaimedKSM = isClaimedKSM;
      });
    },
    setIsClaimedStablecoin: (isClaimedStablecoin) => {
      set(function fetchClaimTotals(state: AllSlices) {
        state.ui.isClaimedStablecoin = isClaimedStablecoin;
      });
    },
  },
});
