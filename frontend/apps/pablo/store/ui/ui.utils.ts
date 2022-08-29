import produce from "immer";
import { UISlice } from "./ui.types";

export const setPolkadotModalVisibility = (
  uiState: UISlice["ui"],
  state: boolean
) => {
  return produce(uiState, (draft) => {
    draft.isPolkadotModalOpen = state;
  });
};

export const setUiSlice = (
  uiState: UISlice["ui"],
  updates: Partial<UISlice["ui"]>
) => {
  return produce(uiState, (draft) => {
    draft.isPolkadotModalOpen= updates.isPolkadotModalOpen ?? uiState.isPolkadotModalOpen;
    draft.isPreviewSupplyModalOpen= updates.isPreviewSupplyModalOpen ?? uiState.isPreviewSupplyModalOpen
    draft.isConfirmSupplyModalOpen= updates.isConfirmSupplyModalOpen ?? uiState.isConfirmSupplyModalOpen
    draft.isConfirmingSupplyModalOpen= updates.isConfirmingSupplyModalOpen ?? uiState.isConfirmingSupplyModalOpen
    draft.isConfirmedSupply= updates.isConfirmedSupply ?? uiState.isConfirmedSupply
    draft.isConfirmingModalOpen= updates.isConfirmingModalOpen ?? uiState.isConfirmingModalOpen
    draft.isSwapSettingsModalOpen= updates.isSwapSettingsModalOpen ?? uiState.isSwapSettingsModalOpen
    draft.isAccountSettingsModalOpen= updates.isAccountSettingsModalOpen ?? uiState.isAccountSettingsModalOpen
    draft.hasTriedEagerConnect= updates.hasTriedEagerConnect ?? uiState.hasTriedEagerConnect
  })
}