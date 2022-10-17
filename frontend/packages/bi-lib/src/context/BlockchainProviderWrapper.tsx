import dynamic from "next/dynamic";
import React from 'react';
import { Provider } from "react-redux";

import { HooksStore } from "../workaround-web3-react-issues-379/HooksStore";
import { BlockchainProvider, BlockchainProviderProps } from "./BlockchainProvider";

const HooksProvider = dynamic(
  () => import('../workaround-web3-react-issues-379/HooksProvider'),
  { ssr: false }
);

export const BlockchainProviderWrapper = (props: BlockchainProviderProps) =>
  <Provider store={HooksStore}>
    <HooksProvider>
      <BlockchainProvider {...props} />
    </HooksProvider>
  </Provider>