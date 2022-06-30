import { TOKENS } from "@/defi/Tokens";
import { Token, XPablo } from "@/defi/types";
import { createSlice } from "@reduxjs/toolkit";
import { bondPoolsData } from "../../utils/bondPoolsData";
import BigNumber from "bignumber.js";

export type BondPoolRow = {
  token1: Token;
  token2?: Token;
  tvl: BigNumber;
  roi: BigNumber;
  rewardsLeft: Array<{
    value: BigNumber;
    token: Token;
  }>;
  volume: BigNumber;
  price: BigNumber;
  pending: BigNumber;
};

export type YourBondPoolRow = {
  token1: Token;
  token2: Token;
  tvl: BigNumber;
  apr: BigNumber;
  bond: Array<{
    value: BigNumber;
    token: Token;
  }>;
  volume: BigNumber;
  vesting_term: number;
  claimable: BigNumber;
  discount: BigNumber;
  price: BigNumber;
  pending: BigNumber;
};

export type UserStakeInfo = {
  balance: BigNumber;
  stakedBalance: BigNumber;
  nextRewardAmount: BigNumber;
  roi: number;
};

export type Overview = {
  totalValueLocked: BigNumber;
  tradingVolume24hrs: BigNumber;
  pabloPrice: BigNumber;
};

export type StakingOverview = {
  totalPBLOLocked: BigNumber,
  totalChaosApy: number,
  totalKsmApy: number,
  totalPicaApy: number,
  totalPabloApy: number,
  totalChaosMinted: BigNumber,
  averageLockMultiplier: number,
  averageLockTime: number,
};

export type ClaimableRewards = {
  ksm: BigNumber,
  pica: BigNumber,
  pablo: BigNumber,
};

export type BondChartData = {
  total: BigNumber,
  change: number,
  series: [number, number][],
};

interface PolkadotState {
  overview: Overview;
  stakingOverview: StakingOverview,
  allBondPools: BondPoolRow[];
  yourBondPools: YourBondPoolRow[];
  userStakeInfo: UserStakeInfo;
  yourXPablos: XPablo[];
  claimableRewards: ClaimableRewards,
  bondPortfolioChartData: BondChartData,
}

const initialState: PolkadotState = {
  overview: {
    totalValueLocked: new BigNumber(66543234),
    tradingVolume24hrs: new BigNumber(12312654),
    pabloPrice: new BigNumber(1.54),
  },
  stakingOverview: {
    totalPBLOLocked: new BigNumber(20356251),
    totalChaosApy: 268,
    totalKsmApy: 58,
    totalPicaApy: 58,
    totalPabloApy: 58,
    totalChaosMinted: new BigNumber(4265),
    averageLockMultiplier: 0.8,
    averageLockTime: 265,
  },
  claimableRewards: {
    ksm: new BigNumber(25.135),
    pica: new BigNumber(55265),
    pablo: new BigNumber(48551),
  },
  allBondPools: [],
  yourBondPools: [
    {
      token1: TOKENS["pica"],
      token2: TOKENS["ksm"],
      tvl: new BigNumber(1500000),
      apr: new BigNumber(5.75),
      bond: [
        {
          token: TOKENS["pica"],
          value: new BigNumber(5000),
        },
        {
          token: TOKENS["ksm"],
          value: new BigNumber(5200),
        },
      ],
      volume: new BigNumber(132500000),
      vesting_term: 5,
      claimable: new BigNumber(500),
      discount: new BigNumber(0.1),
      price: new BigNumber(350.34),
      pending: new BigNumber(20),
    },
    {
      token1: TOKENS["pablo"],
      token2: TOKENS["ksm"],
      tvl: new BigNumber(1500000),
      apr: new BigNumber(5.75),
      bond: [
        {
          token: TOKENS["pica"],
          value: new BigNumber(3340),
        },
      ],
      volume: new BigNumber(132500000),
      vesting_term: 5,
      claimable: new BigNumber(500),
      discount: new BigNumber(0.1),
      price: new BigNumber(350.34),
      pending: new BigNumber(20),
    },
  ],
  bondPortfolioChartData: {
    total: new BigNumber(24546395.04),
    change: 2,
    series: [
      [1644550600000, 20],
      [1644560620928, 45],
      [1644570600000, 40],
      [1644590600000, 100],
    ],
  },
  userStakeInfo: {
    balance: new BigNumber(200),
    stakedBalance: new BigNumber(0),
    nextRewardAmount: new BigNumber(0),
    roi: 8.2,
  },
  yourXPablos: [
    {
      id: 357,
      tokenId: "pablo",
      locked: new BigNumber(34567),
      expiry: 1645345320000,
      multiplier: 1,
      amount: new BigNumber(23309),
      withdrawableAmount: new BigNumber(23309),
    },
    {
      id: 415,
      tokenId: "pablo",
      locked: new BigNumber(3435),
      expiry: 1656547200000,
      multiplier: 1,
      amount: new BigNumber(2330),
      withdrawableAmount: new BigNumber(2330),
    }
  ]
};

export const polkadotSlice = createSlice({
  name: "PolkaDot",
  initialState,
  reducers: {
    addNextDataBondPools: (state, action) => {
      state.allBondPools = [
        ...state.allBondPools,
        ...bondPoolsData.slice(
          action.payload.startIndex,
          action.payload.startIndex + 4
        ),
      ];
    },
  },
});

export const {
  addNextDataBondPools,
} = polkadotSlice.actions;

export default polkadotSlice.reducer;
