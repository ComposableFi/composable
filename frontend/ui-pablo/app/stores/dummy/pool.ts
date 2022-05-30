import BigNumber from "bignumber.js";

export const initPoolData = {
  type: 'Weighted',
  ammId: 'none',
  tokenId1: 'none',
  tokenId2: 'none',
  tokenWeight1: new BigNumber(50),
  tokenWeight2: new BigNumber(50),
  initialSwapFee: new BigNumber(0.3),
} as const;

export const initSupplyData = {
  tokenId1: 'none',
  tokenId2: 'none',
  pooledAmount1: new BigNumber(0),
  pooledAmount2: new BigNumber(0),
  approvedToken1: false,
  approvedToken2: false,
  price1: new BigNumber(10),
  price2: new BigNumber(0.1),
  share: new BigNumber(3.3),
  amount: new BigNumber(1.57),
  balance1: new BigNumber(500.35523),
  balance2: new BigNumber(600.35523),
  confirmed: false,
} as const;