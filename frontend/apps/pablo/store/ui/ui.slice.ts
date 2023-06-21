import create from "zustand";
import { UIState } from "./ui.types";

export const useUiSlice = create(() => ({
  isPolkadotModalOpen: false,
  isOpenPreviewPurchaseModal: false,
  isWrongAmountEnteredModalOpen: false,
  isUnverifiedPoolWarningOpen: false,
  isSwapPreviewModalOpen: false,
  isConfirmSupplyModalOpen: false,
  isPreviewSupplyModalOpen: false,
  isConfirmingSupplyModalOpen: false,
  isTransactionSettingsModalOpen: false,
  isConfirmedSupply: false,
  isConfirmingModalOpen: false,
  isSwapSettingsModalOpen: false,
  isAccountSettingsModalOpen: false,
  hasTriedEagerConnect: false,
}));

export const setUiState = (updatedState: Partial<UIState>) => useUiSlice.setState((state) => ({
  ...state,
  ...updatedState
}))