import React from 'react';
import { DotsamaContext } from '../DotSamaContext';
import {
  RelayChainId,
} from '../types';

export const useRelayChainApi = (relaychainId: RelayChainId) => {
  const { relaychainProviders } = React.useContext(DotsamaContext);
  return relaychainProviders[relaychainId];
};

