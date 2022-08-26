import { Signer } from '@polkadot/api/types';
import React from 'react';
import { DotsamaContext } from '../DotSamaContext';
import {
  ConntectedAccount,
  DotSamaContext,
  ParachainId,
  RelayChainId,
} from '../types';

export const useDotSamaContext = (): DotSamaContext => {
  return React.useContext(DotsamaContext);
};

export const useParachainApi = (parachainId: ParachainId) => {
  const { parachainProviders } = React.useContext(DotsamaContext);
  return parachainProviders[parachainId];
};

export const useRelayChainApi = (relaychainId: RelayChainId) => {
  const { relaychainProviders } = React.useContext(DotsamaContext);
  return relaychainProviders[relaychainId];
};

export const useSelectedAccount = (
  parachainId: ParachainId
): ConntectedAccount | undefined => {
  const { selectedAccount, parachainProviders } = React.useContext(
    DotsamaContext
  );
  const { accounts } = parachainProviders[parachainId];
  return selectedAccount !== -1 ? accounts[selectedAccount] : undefined;
};

export const useSigner = (): Signer | undefined => {
  const { signer } = React.useContext(
    DotsamaContext
  );

  return signer;
}