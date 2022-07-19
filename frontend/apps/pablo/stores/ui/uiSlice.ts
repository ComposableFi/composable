import { Message } from "@/components/types";
import { createSlice, PayloadAction } from "@reduxjs/toolkit";

interface UIState {
  message: Message;
  isPolkadotModalOpen: boolean;
  isConfirmSupplyModalOpen: boolean;
  isPreviewSupplyModalOpen: boolean;
  isConfirmingSupplyModalOpen: boolean;
  isConfirmedSupply: boolean;
  isConfirmingModalOpen: boolean;
  isTransactionSettingsModalOpen: boolean;
  isAccountSettingsModalOpen: boolean;
  isSwapPreviewModalOpen: boolean;
  isWrongAmountEnteredModalOpen: boolean;
}

const initialState: UIState = {
  message: {},
  isPolkadotModalOpen: false,
  isConfirmSupplyModalOpen: false,
  isPreviewSupplyModalOpen: false,
  isConfirmingSupplyModalOpen: false,
  isConfirmedSupply: false,
  isConfirmingModalOpen: false,
  isTransactionSettingsModalOpen: false,
  isAccountSettingsModalOpen: false,
  isSwapPreviewModalOpen: false,
  isWrongAmountEnteredModalOpen: false,
};

export const uiSlice = createSlice({
  name: "UI",
  initialState,
  reducers: {
    setMessage: (state, action) => {
      state.message = action.payload;
    },
    openPolkadotModal: (state) => {
      state.isPolkadotModalOpen = true;
    },
    closePolkadotModal: (state) => {
      state.isPolkadotModalOpen = false;
    },
    openConfirmSupplyModal: (state) => {
      state.isConfirmSupplyModalOpen = true;
    },
    closeConfirmSupplyModal: (state) => {
      state.isConfirmSupplyModalOpen = false;
    },
    openPreviewSupplyModal: (state) => {
      state.isPreviewSupplyModalOpen = true;
    },
    closePreviewSupplyModal: (state) => {
      state.isPreviewSupplyModalOpen = false;
    },
    openConfirmingSupplyModal: (state) => {
      state.isConfirmingSupplyModalOpen = true;
    },
    closeConfirmingSupplyModal: (state) => {
      state.isConfirmingSupplyModalOpen = false;
    },
    setConfirmedSupply: (state, action) => {
      state.isConfirmedSupply = action.payload;
    },
    openConfirmingModal: (state) => {
      state.isConfirmingModalOpen = true;
    },
    closeConfirmingModal: (state) => {
      state.isConfirmingModalOpen = false;
    },
    openTransactionSettingsModal: (state) => {
      state.isTransactionSettingsModalOpen = true;
    },
    closeTransactionSettingsModal: (state) => {
      state.isTransactionSettingsModalOpen = false;
    },
    openAccountSettingsModal: (state) => {
      state.isAccountSettingsModalOpen = true;
    },
    closeAccountSettingsModal: (state) => {
      state.isAccountSettingsModalOpen = false;
    },
    openSwapPreviewModal: (state) => {
      state.isSwapPreviewModalOpen = true;
    },
    closeSwapPreviewModal: (state) => {
      state.isSwapPreviewModalOpen = false;
    },
    openWrongAmountEnteredModal: (state) => {
      state.isWrongAmountEnteredModalOpen = true;
    },
    closeWrongAmountEnteredModal: (state) => {
      state.isWrongAmountEnteredModalOpen = false;
    },
  },
});

export const {
  setMessage,
  openPolkadotModal,
  closePolkadotModal,
  openConfirmSupplyModal,
  closeConfirmSupplyModal,
  openPreviewSupplyModal,
  closePreviewSupplyModal,
  openConfirmingSupplyModal,
  closeConfirmingSupplyModal,
  setConfirmedSupply,
  openConfirmingModal,
  closeConfirmingModal,
  openTransactionSettingsModal,
  closeTransactionSettingsModal,
  openAccountSettingsModal,
  closeAccountSettingsModal,
  openSwapPreviewModal,
  closeSwapPreviewModal,
  openWrongAmountEnteredModal,
  closeWrongAmountEnteredModal,
} = uiSlice.actions;

export default uiSlice.reducer;
