import { ExtrinsicMetadata, ExtrinsicStatus } from "@/lib/types";

/* The Slice type */
export interface ExtrinsicSlice {
  extrinsics: {
    [txHash: string]: ExtrinsicMetadata;
  };
  addExtrinsic: (
    transactionHash: string,
    extrinsicCall: ExtrinsicMetadata
  ) => void;
  updateExtrinsicStatus: (
    transactionHash: string,
    status: ExtrinsicStatus
  ) => void;
  updateExtrinsicError: (transactionHash: string, errorMessage: string) => void;
  addBlockHash: (transactionHash: string, blockHash: string) => void;
}
