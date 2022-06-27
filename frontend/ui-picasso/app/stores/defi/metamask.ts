import { NamedSet } from "zustand/middleware";
import { AllSlices, StoreSlice } from "../types";

export type Account = {
  address: string;
};

interface MetamaskState {
  connected: boolean;
  connecting: boolean;
  eligible: boolean;
  account: Account;
  availableToClaim: number;
}

const initialState: MetamaskState = {
  connected: false,
  connecting: false,
  account: {
    address: "0x729e86ed5614348d66996f0E23f28012eaCb0D17",
  },
  eligible: true,
  availableToClaim: 122,
};

export interface MetamaskSlice {
  metamask: MetamaskState & {
    connectMetamaskWallet: () => void;
    waitOnMetamaskWallet: () => void;
    disconnectMetamaskWallet: () => void;
    setAvailableToClaim: (availableToClaim: number) => void;
  };
}

export const createMetamaskSlice: StoreSlice<MetamaskSlice> = (
  set: NamedSet<MetamaskSlice>
) => ({
  metamask: {
    ...initialState,
    connectMetamaskWallet: () => {
      set((state: AllSlices) => {
        state.metamask.connected = true;
      });
    },
    waitOnMetamaskWallet: () => {
      set((state: AllSlices) => {
        state.metamask.connecting = true;
      });
    },
    disconnectMetamaskWallet: () => {
      set((state: AllSlices) => {
        state.metamask.connected = false;
      });
    },
    setAvailableToClaim: (availableToClaim: number) => {
      set((state: AllSlices) => {
        state.metamask.availableToClaim = availableToClaim;
      });
    },
  },
});
