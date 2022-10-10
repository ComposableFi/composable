import { useMemo } from "react";
import {
  ExtrinsicMetadata,
  ExtrinsicStatus,
  useExtrinsicStore,
} from "../store/extrinsics";

export function useExtrinsics() {
  return useExtrinsicStore((state) => state.extrinsics);
}

function isPending(extrinsicStatus: ExtrinsicStatus): boolean {
  return extrinsicStatus !== "isFinalized" && extrinsicStatus !== "Error";
}

export const usePendingExtrinsic = (
  method: string,
  section: string,
  sender: string
): boolean => {
  const extrinsics = useExtrinsicStore((state) => state.extrinsics);

  return useMemo(() => {
    const sortedTxs = Object.values(extrinsics).sort((a, b) => {
      return a.timestamp - b.timestamp;
    });

    for (const tx of sortedTxs) {
      if (
        tx.method === method &&
        section === tx.section &&
        tx.sender === sender
      ) {
        if (isPending(tx.status)) {
          return true;
        }
      }
    }

    return false;
  }, [extrinsics]); // eslint-disable-line react-hooks/exhaustive-deps
};

export const useExtrinsicCalls = (
  method: string,
  section: string,
  sender: string
): ExtrinsicMetadata[] => {
  const extrinsics = useExtrinsicStore((state) => state.extrinsics);
  return useMemo(() => {
    let calls = [];

    for (const tx of Object.values(extrinsics)) {
      if (
        tx.method === method &&
        section === tx.section &&
        tx.sender === sender
      ) {
        calls.push({ ...tx });
      }
    }

    return calls;
  }, [extrinsics]); // eslint-disable-line react-hooks/exhaustive-deps
};
