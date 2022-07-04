import { ExtrinsicMetadata, ExtrinsicSlice, ExtrinsicStatus } from './extrinsics.types';
export declare const putTransactionData: (transactions: ExtrinsicSlice['extrinsics'], txHash: string, data: ExtrinsicMetadata) => {
    [txHash: string]: ExtrinsicMetadata;
};
export declare const putTrasactionStatus: (transactions: ExtrinsicSlice['extrinsics'], txHash: string, status: ExtrinsicStatus) => {
    [txHash: string]: ExtrinsicMetadata;
};
export declare const putTransactionError: (transactions: ExtrinsicSlice['extrinsics'], txHash: string, errorMessage: string) => {
    [txHash: string]: ExtrinsicMetadata;
};
export declare const putBlockHash: (transactions: ExtrinsicSlice['extrinsics'], txHash: string, blockHash: string) => {
    [txHash: string]: ExtrinsicMetadata;
};
