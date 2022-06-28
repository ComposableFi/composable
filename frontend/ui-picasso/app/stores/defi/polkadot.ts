import { Token, TOKENS } from "@/defi/Tokens";

import { createSlice } from "@reduxjs/toolkit";

// TODO: [defi] edit values accordingly to your needs
export type Account = {
  label: string;
  value: string;
};

export type Asset = {
  token: Token;
  price: number;
  balance: number;
  value: number;
  change_24hr: number;
};
export type StakingAsset = {
  token: Token;
  toToken: Token;
  price: number;
  balance: number;
  value: number;
  change_24hr: number;
};
export type StakingInfo = {
  deposits: string;
  apy: string;
  totalStaked: string;
  balance: number;
  stakedBalance: number;
  nextRewardAmount: number;
  roi: number;
};

const initialState: {
  myStakingAssets: { pablo: any[]; picasso: any[] };
  staking: {
    nextRewardAmount: number;
    balance: number;
    stakedBalance: number;
    apy: string;
    roi: number;
    deposits: string;
    totalStaked: string;
  };
  assets: (
    | {
        balance: number;
        price: number;
        change_24hr: number;
        value: number;
        token: Token;
      }
    | {
        balance: number;
        price: number;
        change_24hr: number;
        value: number;
        token: Token;
      }
  )[];
  selectedAccount: null;
} = {
  selectedAccount: null,
  assets: [
    {
      token: TOKENS["pica"],
      price: 1.43,
      balance: 4534,
      value: 46187,
      change_24hr: 0.34,
    },
    {
      token: TOKENS["ksm"],
      price: 189,
      balance: 42,
      value: 984.98,
      change_24hr: -0.12,
    },
  ],
  myStakingAssets: {
    picasso: [],
    pablo: [],
  },
  staking: {
    deposits: "$0",
    apy: "2,624%",
    totalStaked: "$66.3K",
    balance: 200,
    stakedBalance: 0,
    nextRewardAmount: 0,
    roi: 8.2,
  },
};

export const polkadotSlice = createSlice({
  name: "PolkaDot",
  initialState,
  reducers: {},
});

export default polkadotSlice.reducer;
