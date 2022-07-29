import React, { useMemo } from 'react';
import Executor from './Executor';
import useStore from './store/useStore';
/**
 * As zustand useStore is a hook
 * we need to create a context and wrap
 * executor in a provider to be able to
 * use useStore methods via executor
 *
 * exectuor would expose execute and executeUnsigned
 * methods to be able to execute extrsinsic calls
 */
const ExecutorContext = React.createContext({
  executor: undefined as Executor | undefined,
});

export const ExecutorProvider = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  /**
   * Use store updaters
   * from zustand store
   */
  const {
    addExtrinsic,
    addBlockHash,
    updateExtrinsicStatus,
    updateExtrinsicError,
  } = useStore();
  /**
   * Create and memoize executor
   */
  const executor = useMemo<Executor>(() => {
    return new Executor(
      addExtrinsic,
      addBlockHash,
      updateExtrinsicStatus,
      updateExtrinsicError
    );
  }, [addExtrinsic, addBlockHash, updateExtrinsicStatus, updateExtrinsicError]);

  return (
    <ExecutorContext.Provider
      value={{
        executor,
      }}
    >
      {children}
    </ExecutorContext.Provider>
  );
};

/**
 * Hook that returns an extrinsics executor
 * @returns Executor
 */
export const useExecutor = (): Executor | undefined => {
  return React.useContext(ExecutorContext).executor;
};
