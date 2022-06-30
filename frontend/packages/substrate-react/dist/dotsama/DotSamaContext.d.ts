import React from 'react';
import { DotSamaContext, ParachainId } from './types';
export declare const DotsamaContext: React.Context<DotSamaContext>;
export declare const DotSamaContextProvider: ({ supportedParachains, children, appName, }: {
    appName: string;
    supportedParachains: {
        chainId: ParachainId;
        rpcUrl: string;
        rpc: any;
        types: any;
    }[];
    children: React.ReactNode;
}) => JSX.Element;
