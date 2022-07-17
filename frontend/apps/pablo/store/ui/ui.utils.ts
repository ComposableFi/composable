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