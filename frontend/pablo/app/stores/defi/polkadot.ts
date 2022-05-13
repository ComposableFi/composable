import { TOKENS } from "@/defi/Tokens";
import { Token, XPablo } from "@/defi/types";
import { createSlice } from "@reduxjs/toolkit";
import { liquidityPoolsData } from "../../utils/liquidityPoolsData";
import { bondPoolsData } from "../../utils/bondPoolsData";
import BigNumber from "bignumber.js";

// TODO: [defi] edit values accordingly to your needs

export type Asset = {
  token: Token;
  price: BigNumber;
  balance: BigNumber;
  value: BigNumber;
  change24hr: BigNumber;
};

export type LiquidityPoolRow = {
  token1: Token;
  token2: Token;
  tvl: BigNumber;
  apr: BigNumber;
  rewardsLeft: Array<{
    value: BigNumber;
    token: Token;
  }>;
  volume: BigNumber;
  price?: BigNumber;
};

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
}

interface PolkadotState {
  assets: Asset[];
  overview: Overview;
  allLiquidityPools: LiquidityPoolRow[];
  yourLiquidityPools: LiquidityPoolRow[];
  allBondPools: BondPoolRow[];
  yourBondPools: YourBondPoolRow[];
  userStakeInfo: UserStakeInfo;
  yourXPablos: XPablo[];
}

const initialState: PolkadotState = {
  assets: [
    {
      token: TOKENS["pica"],
      price: new BigNumber(1.43),
      balance: new BigNumber(4534),
      value: new BigNumber(46187),
      change24hr: new BigNumber(0.34),
    },
    {
      token: TOKENS["ksm"],
      price: new BigNumber(189),
      balance: new BigNumber(42),
      value: new BigNumber(984.98),
      change24hr: new BigNumber(-0.12),
    },
  ],
  overview: {
    totalValueLocked: new BigNumber(66543234),
    tradingVolume24hrs: new BigNumber(12312654),
    pabloPrice: new BigNumber(1.54),
  },
  allLiquidityPools: [],
  yourLiquidityPools: [
    {
      token1: TOKENS["pica"],
      token2: TOKENS["ksm"],
      tvl: new BigNumber(1500000),
      apr: new BigNumber(5.75),
      rewardsLeft: [
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
      price: new BigNumber(0.1),
    },
    {
      token1: TOKENS["pablo"],
      token2: TOKENS["ksm"],
      tvl: new BigNumber(1500000),
      apr: new BigNumber(5.75),
      rewardsLeft: [
        {
          token: TOKENS["pica"],
          value: new BigNumber(3340),
        },
        {
          token: TOKENS["ksm"],
          value: new BigNumber(3453.49),
        },
      ],
      volume: new BigNumber(132500000),
      price: new BigNumber(0.1),
    },
  ],
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
    },
  ],
  userStakeInfo: {
    balance: new BigNumber(200),
    stakedBalance: new BigNumber(0),
    nextRewardAmount: new BigNumber(0),
    roi: 8.2,
  },
  yourXPablos: [
    {
      tokenId: "pablo",
      locked: new BigNumber(1000000),
      expiry: 1645345320000,
      muliplier: 1,
      amount: new BigNumber(1000000),
    }
  ]
};

export const polkadotSlice = createSlice({
  name: "PolkaDot",
  initialState,
  reducers: {
    addNextDataLiquidityPools: (state, action) => {
      state.allLiquidityPools = [
        ...state.allLiquidityPools,
        ...liquidityPoolsData.slice(
          action.payload.startIndex,
          action.payload.startIndex + 4
        ),
      ];
    },
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
  addNextDataLiquidityPools,
  addNextDataBondPools,
} = polkadotSlice.actions;

export default polkadotSlice.reducer;
