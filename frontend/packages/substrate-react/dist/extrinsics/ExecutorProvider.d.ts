import React from 'react';
import Executor from './Executor';
export declare const ExecutorProvider: ({ children, }: {
    children: React.ReactNode;
}) => JSX.Element;
/**
 * Hook that returns an extrinsics executor
 * @returns Executor
 */
export declare const useExecutor: () => Executor | undefined;
