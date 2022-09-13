import {
  ExtrinsicMetadata,
  ExtrinsicSlice,
  ExtrinsicStatus,
} from "./extrinsics.types";
import {
  putBlockHash,
  putTransactionData,
  putTransactionError,
  putTransactionStatus,
} from "./extrinsics.utils";
import create from "zustand";
import { immer } from "zustand/middleware/immer";
import { devtools } from "zustand/middleware";

export const useExtrinsicStore = create<ExtrinsicSlice>()(
  immer(
    devtools((set) => ({
      extrinsics: {},
      addExtrinsic: (
        transactionHash: string,
        extrinsicCall: Omit<ExtrinsicMetadata, "dispatchError">
      ) =>
        set((state: ExtrinsicSlice) => {
          state.extrinsics = putTransactionData(
            state.extrinsics,
            transactionHash,
            extrinsicCall
          );
        }),
      addBlockHash: (transactionHash: string, blockHash: string) =>
        set((state: ExtrinsicSlice) => {
          state.extrinsics = putBlockHash(
            state.extrinsics,
            transactionHash,
            blockHash
          );
        }),
      updateExtrinsicStatus: (
        transactionHash: string,
        extrinsicStatus: ExtrinsicStatus
      ) =>
        set((state: ExtrinsicSlice) => {
          state.extrinsics = putTransactionStatus(
            state.extrinsics,
            transactionHash,
            extrinsicStatus
          );
        }),
      updateExtrinsicError: (transactionHash: string, errorMessage: string) =>
        set((state: ExtrinsicSlice) => {
          state.extrinsics = putTransactionError(
            state.extrinsics,
            transactionHash,
            errorMessage
          );
        }),
    }))
  )
);
