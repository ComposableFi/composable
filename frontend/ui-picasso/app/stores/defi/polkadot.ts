import { Token, TOKENS } from "@/defi/Tokens";

import { createSlice } from "@reduxjs/toolkit";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";

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
export type BondingAsset = {
  token: Token;
  toToken: Token;
  claimable: number;
  pending: number;
  vesting_time: string;
};

export type AllBondsAsset = {
  token: Token;
  toToken: Token;
  price: number;
  roi: number;
  totalPurchased: string;
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
interface PolkadotState {
  assets: Asset[];
  myStakingAssets: {
    picasso: Asset[];
    pablo: StakingAsset[];
  };
  myBondingAssets: {
    picasso: BondingAsset[];
    pablo: BondingAsset[];
  };
  selectedAccount: Account | null;
  allBonds: AllBondsAsset[];
  bonds: BondOffer[];
  staking: StakingInfo;
}

const initialState: { myStakingAssets: { pablo: any[]; picasso: any[] }; staking: { nextRewardAmount: number; balance: number; stakedBalance: number; apy: string; roi: number; deposits: string; totalStaked: string }; assets: ({ balance: number; price: number; change_24hr: number; value: number; token: Token } | { balance: number; price: number; change_24hr: number; value: number; token: Token })[]; bonds: any[]; selectedAccount: null; myBondingAssets: { pablo: any[]; picasso: any[] } } = {
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
    picasso: [
    ],
    pablo: [

    ],
  },
  myBondingAssets: {
    picasso: [

    ],
    pablo: [

    ],
  },
  bonds: [

  ],
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
