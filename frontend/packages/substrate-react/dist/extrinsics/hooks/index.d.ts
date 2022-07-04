import { ExtrinsicMetadata, ExtrinsicSlice } from '../store/extrinsics/extrinsics.types';
export declare const useExtrinsics: () => ExtrinsicSlice['extrinsics'];
export declare const usePendingExtrinsic: (method: string, section: string, sender: string) => boolean;
export declare const useExtrinsicCalls: (method: string, section: string, sender: string) => ExtrinsicMetadata[];
