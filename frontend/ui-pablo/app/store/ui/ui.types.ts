export interface UIState {
    isPolkadotModalOpen: boolean;
    isPreviewSupplyModalOpen: boolean;
    isConfirmSupplyModalOpen: boolean;
    isConfirmingSupplyModalOpen: boolean;
    isConfirmedSupply: boolean;
    isConfirmingModalOpen: boolean;
    isSwapSettingsModalOpen: boolean;
    isAccountSettingsModalOpen: boolean;
}

export interface UISlice {
    ui: UIState,
    openPolkadotModal: () => void,
    closePolkadotModal: () => void
}