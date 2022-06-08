import { StoreSlice } from "../types";
import { UISlice } from "./ui.types";
import { setPolkadotModalVisibility } from "./ui.utils";

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
  },
  openPolkadotModal: () =>
    set((prev: UISlice) => ({
      ui: setPolkadotModalVisibility(prev.ui, true),
    })),
  closePolkadotModal: () =>
    set((prev: UISlice) => ({
      ui: setPolkadotModalVisibility(prev.ui, false),
    })),
});

export default createUiSlice;
