import { NamedSet } from "zustand/middleware";
import { StoreSlice } from "@/stores/types";

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
    connected: false,
    connecting: false,
    account: {
      address: "0x729e86ed5614348d66996f0E23f28012eaCb0D17",
    },
    eligible: true,
    availableToClaim: 122,
    // connectMetamaskWallet: () => {
    //   set((state: AllSlices) => {
    //     state.metamask.connected = true;
    //   });
    // },
    connectMetamaskWallet: () =>
      set((state) => ({
        metamask: {
          ...state.metamask,
          connected: true,
        },
      })),
    waitOnMetamaskWallet: () =>
      set((state) => ({
        metamask: {
          ...state.metamask,
          connecting: true,
        },
      })),

    disconnectMetamaskWallet: () =>
      set((state) => ({
        metamask: {
          ...state.metamask,
          connected: false,
        },
      })),
    setAvailableToClaim: (availableToClaim: number) =>
      set((state) => ({
        metamask: {
          ...state.metamask,
          availableToClaim,
        },
      })),
  },
});
