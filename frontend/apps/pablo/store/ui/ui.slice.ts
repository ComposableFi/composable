import { StoreSlice } from "../types";
import { UISlice } from "./ui.types";
import { setPolkadotModalVisibility, setUiSlice } from "./ui.utils";

const createUiSlice: StoreSlice<UISlice> = (set) => ({
  ui: {
    isPolkadotModalOpen: false,
    isConfirmSupplyModalOpen: false,
    isPreviewSupplyModalOpen: false,
    isConfirmingSupplyModalOpen: false,
    isConfirmedSupply: false,
    isConfirmingModalOpen: false,
    isSwapSettingsModalOpen: false,
    isAccountSettingsModalOpen: false,
    hasTriedEagerConnect: false,
  },
  openPolkadotModal: () =>
    set((prev: UISlice) => ({
      ui: setPolkadotModalVisibility(prev.ui, true),
    })),
  closePolkadotModal: () =>
    set((prev: UISlice) => ({
      ui: setPolkadotModalVisibility(prev.ui, false),
    })),
  setUiState: (state) =>
    set((prev: UISlice) => {
      ui: setUiSlice(prev.ui, state);
    }),
});

export default createUiSlice;
