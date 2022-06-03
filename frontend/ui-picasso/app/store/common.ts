import { NamedSet } from "zustand/middleware";
import { AppState, StoreSlice } from "./types";

interface CommonProps {
  status: string;
}

export interface CommonSlice {
  common: CommonProps & {
    setStatus: (status: string) => void;
  };
}

export const createCommonSlice: StoreSlice<CommonSlice> = (
  set: NamedSet<CommonSlice>
) => ({
  common: {
    status: "",

    setStatus: async (status: string) => {
      set(function setStatus(state: AppState) {
        state.common.status = status;
      });
    },
  },
});
