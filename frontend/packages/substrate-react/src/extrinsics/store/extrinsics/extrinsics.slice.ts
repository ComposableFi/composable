import { StoreSlice } from '../types';
import {
  ExtrinsicSlice,
  ExtrinsicMetadata,
  ExtrinsicStatus,
} from './extrinsics.types';
import {
  putTransactionData,
  putBlockHash,
  putTrasactionStatus,
  putTransactionError,
} from './extrinsics.utils';

const createExtrinsicsSlice: StoreSlice<ExtrinsicSlice> = set => ({
  extrinsics: {},
  addExtrinsic: (
    transactionHash: string,
    extrinsicCall: Omit<ExtrinsicMetadata, 'dispatchError'>
  ) =>
    set((prev: ExtrinsicSlice) => ({
      extrinsics: putTransactionData(
        prev.extrinsics,
        transactionHash,
        extrinsicCall
      ),
    })),
  addBlockHash: (transactionHash: string, blockHash: string) =>
    set((prev: ExtrinsicSlice) => ({
      extrinsics: putBlockHash(prev.extrinsics, transactionHash, blockHash),
    })),
  updateExtrinsicStatus: (
    transactionHash: string,
    extrinsicStatus: ExtrinsicStatus
  ) =>
    set((prev: ExtrinsicSlice) => ({
      extrinsics: putTrasactionStatus(
        prev.extrinsics,
        transactionHash,
        extrinsicStatus
      ),
    })),
  updateExtrinsicError: (transactionHash: string, errorMessage: string) =>
    set((prev: ExtrinsicSlice) => ({
      extrinsics: putTransactionError(
        prev.extrinsics,
        transactionHash,
        errorMessage
      ),
    })),
});

export default createExtrinsicsSlice;
