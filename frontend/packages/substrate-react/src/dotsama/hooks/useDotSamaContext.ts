import React from 'react';
import { DotsamaContext } from '../DotSamaContext';
import {
  DotSamaContext,
} from '../types';

export const useDotSamaContext = (): DotSamaContext => {
  return React.useContext(DotsamaContext);
};
