import { PoolDetails } from "@/defi/types";
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

export const selectedPoolData = {
  tokenId1: "pica",
  tokenId2: "ksm",
  tokenWeight1: new BigNumber(50),
  tokenWeight2: new BigNumber(50),
  initialSwapFee: new BigNumber(0.3),
  poolValue: new BigNumber(325300651),
  poolAmount: new BigNumber(3353),
  rewardValue: new BigNumber(500),
  rewardsLeft: [
    {
      tokenId: "pablo",
      value: new BigNumber(5000),
    },
    {
      tokenId: "chaos",
      value: new BigNumber(5200),
    },
  ],
  volume: new BigNumber(325651),
  fee24h: new BigNumber(1563),
  apr: 14.63,
  transactions24h: 119,
  tvlChartData: {
    series: [
      [1644550600000, 20],
      [1644560620928, 45],
      [1644570600000, 40],
      [1644590600000, 100],
    ],
    timeSlots: ["7:00am", "10:00am", "1:00pm", "3:00pm", "5:00pm"],
  },
} as PoolDetails;

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