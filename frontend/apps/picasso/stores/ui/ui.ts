import { StoreSlice } from "@/stores/types";

interface UIState {
  isMetamaskModalOpen: boolean;
  isPolkadotModalOpen: boolean;
  isConfirmingKSM: boolean;
  isConfirmingStablecoin: boolean;
  isClaimingKSM: boolean;
  isClaimingStablecoin: boolean;
  isClaimedKSM: boolean;
  isClaimedStablecoin: boolean;
  stakingRewards: {
    stakeTab: number;
    isBurnModalOpen: boolean;
    agreedSlash: boolean;
    ratio: number;
  };
}

const initialState: UIState = {
  isMetamaskModalOpen: false,
  isPolkadotModalOpen: false,
  isConfirmingKSM: false,
  isConfirmingStablecoin: false,
  isClaimingStablecoin: false,
  isClaimingKSM: false,
  isClaimedKSM: false,
  isClaimedStablecoin: false,
  stakingRewards: {
    stakeTab: 0,
    isBurnModalOpen: false,
    agreedSlash: false,
    ratio: 50,
  },
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
    setIsClaimedKSM: (isClaimedKSM: boolean) => void;
    setIsClaimedStablecoin: (isClaimedStablecoin: boolean) => void;
    setStakingTab: (tabIndex: number) => void;
    setBurnModalState: (state: boolean) => void;
    setAgreedSlash: (agreed: boolean) => void;
    setRatio: (ratio: number) => void;
  };
}

export const createUISlice: StoreSlice<UISlice> = (set) => ({
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
    setStakingTab: (tabIndex: number) => {
      set((state) => {
        state.ui.stakingRewards.stakeTab = tabIndex;
      });
    },
    setBurnModalState: (modalState: boolean) => {
      set((state) => {
        state.ui.stakingRewards.isBurnModalOpen = modalState;
      });
    },
    setAgreedSlash: (agreed: boolean) => {
      set((state) => {
        state.ui.stakingRewards.agreedSlash = agreed;
      });
    },
    setRatio: (ratio: number) => {
      set((state) => {
        state.ui.stakingRewards.ratio = ratio;
      });
    },
  },
});
