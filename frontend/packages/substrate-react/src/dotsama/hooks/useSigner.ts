import { Signer } from '@polkadot/api/types';
import React from 'react';
import { DotsamaContext } from '../DotSamaContext';

export const useSigner = (): Signer | undefined => {
    const { signer } = React.useContext(
      DotsamaContext
    );
  
    return signer;
  }