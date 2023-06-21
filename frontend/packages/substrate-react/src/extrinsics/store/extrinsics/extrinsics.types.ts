/**
 * States of an extrinsic call
 * extracted from polkadot.js
 */
export type ExtrinsicStatus =
  | 'isReady'
  | 'isBroadcast'
  | 'isInBlock'
  | 'isFinalized'
  | 'Error';

/**
 * Actual extrinsic call data
 * that will be provided to the
 * application using this package
 */
export interface ExtrinsicMetadata {
  hash: string;
  method: string;
  section: string;
  sender: string;
  timestamp: number;
  args: { [paramId: string]: any };
  status: ExtrinsicStatus;
  dispatchError?: string;

  isSigned: boolean;
  blockHash?: string;
}

/* The Slice type */
export interface ExtrinsicSlice {
  extrinsics: {
    [txHash: string]: ExtrinsicMetadata;
  };
  addExtrinsic: (
    transactionHash: string,
    extrinsicCall: Omit<ExtrinsicMetadata, 'dispatchError'>
  ) => void;
  updateExtrinsicStatus: (
    transactionHash: string,
    status: ExtrinsicStatus
  ) => void;
  updateExtrinsicError: (transactionHash: string, errorMessage: string) => void;
  addBlockHash: (transactionHash: string, blockHash: string) => void;
}
