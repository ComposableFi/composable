import produce from 'immer';
import {
  ExtrinsicMetadata,
  ExtrinsicSlice,
  ExtrinsicStatus,
} from './extrinsics.types';

export const putTransactionData = (
  transactions: ExtrinsicSlice['extrinsics'],
  txHash: string,
  data: ExtrinsicMetadata
) => {
  return produce(transactions, draft => {
    draft[txHash] = data;
  });
};

export const putTransactionStatus = (
  transactions: ExtrinsicSlice['extrinsics'],
  txHash: string,
  status: ExtrinsicStatus
) => {
  return produce(transactions, draft => {
    if (draft[txHash]) {
      draft[txHash].status = status;
    }
  });
};

export const putTransactionError = (
  transactions: ExtrinsicSlice['extrinsics'],
  txHash: string,
  errorMessage: string
) => {
  return produce(transactions, draft => {
    if (draft[txHash]) {
      draft[txHash].status = 'Error';
      draft[txHash].dispatchError = errorMessage;
    }
  });
};

export const putBlockHash = (
  transactions: ExtrinsicSlice['extrinsics'],
  txHash: string,
  blockHash: string
) => {
  return produce(transactions, draft => {
    if (draft[txHash]) {
      draft[txHash].status = 'isInBlock';
      draft[txHash].blockHash = blockHash;
    }
  });
};
