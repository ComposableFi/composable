import { useMemo } from 'react';
import {
  ExtrinsicMetadata,
  ExtrinsicSlice,
  ExtrinsicStatus,
} from '../store/extrinsics/extrinsics.types';
import useStore from '../store/useStore';

export const useExtrinsics = (): ExtrinsicSlice['extrinsics'] => {
  const { extrinsics } = useStore();
  return extrinsics;
};

function isPending(extrinsicStatus: ExtrinsicStatus): boolean {
  if (extrinsicStatus !== 'isFinalized' && extrinsicStatus !== 'Error') {
    return true;
  }
  return false;
}

export const usePendingExtrinsic = (
  method: string,
  section: string,
  sender: string
): boolean => {
  const { extrinsics } = useStore();

  let _isPendingExtrinsic = useMemo(() => {
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

  return _isPendingExtrinsic;
};

export const useExtrinsicCalls = (
  method: string,
  section: string,
  sender: string
): ExtrinsicMetadata[] => {
  const { extrinsics } = useStore();

  const extrinsicCalls = useMemo(() => {
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

  return extrinsicCalls;
};
