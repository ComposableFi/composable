declare const useStore: import("zustand").UseBoundStore<{
    extrinsics: {
        [txHash: string]: import("./extrinsics/extrinsics.types").ExtrinsicMetadata;
    };
    addExtrinsic: (transactionHash: string, extrinsicCall: Pick<import("./extrinsics/extrinsics.types").ExtrinsicMetadata, "hash" | "method" | "section" | "sender" | "timestamp" | "args" | "status" | "isSigned" | "blockHash">) => void;
    updateExtrinsicStatus: (transactionHash: string, status: import("./extrinsics/extrinsics.types").ExtrinsicStatus) => void;
    updateExtrinsicError: (transactionHash: string, errorMessage: string) => void;
    addBlockHash: (transactionHash: string, blockHash: string) => void;
}, import("zustand").StoreApi<{
    extrinsics: {
        [txHash: string]: import("./extrinsics/extrinsics.types").ExtrinsicMetadata;
    };
    addExtrinsic: (transactionHash: string, extrinsicCall: Pick<import("./extrinsics/extrinsics.types").ExtrinsicMetadata, "hash" | "method" | "section" | "sender" | "timestamp" | "args" | "status" | "isSigned" | "blockHash">) => void;
    updateExtrinsicStatus: (transactionHash: string, status: import("./extrinsics/extrinsics.types").ExtrinsicStatus) => void;
    updateExtrinsicError: (transactionHash: string, errorMessage: string) => void;
    addBlockHash: (transactionHash: string, blockHash: string) => void;
}>>;
export default useStore;
