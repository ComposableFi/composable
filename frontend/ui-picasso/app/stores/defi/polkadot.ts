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
  staking: StakingInfo;
}

const initialState: PolkadotState = {
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
    pablo: [
      {
        token: TOKENS["pica"],
        toToken: TOKENS["ksm"],
        price: 1.43,
        balance: 4534,
        value: 46187,
        change_24hr: 0.34,
      },
      {
        token: TOKENS["ksm"],
        toToken: TOKENS["pica"],
        price: 189,
        balance: 42,
        value: 984.98,
        change_24hr: -0.12,
      },
    ],
  },
  myBondingAssets: {
    picasso: [
      {
        token: TOKENS["ksm"],
        toToken: TOKENS["pica"],
        claimable: 543,
        pending: 123,
        vesting_time: "4D 2H 43M",
      },
      {
        token: TOKENS["pica"],
        toToken: TOKENS["ksm"],
        claimable: 543,
        pending: 123,
        vesting_time: "4D 2H 43M",
      },
    ],
    pablo: [
      {
        token: TOKENS["ksm"],
        toToken: TOKENS["pica"],
        claimable: 543,
        pending: 123,
        vesting_time: "4D 2H 43M",
      },
      {
        token: TOKENS["pica"],
        toToken: TOKENS["ksm"],
        claimable: 543,
        pending: 123,
        vesting_time: "4D 2H 43M",
      },
    ],
  },
  bonds: [
    {
      token: TOKENS["ksm"],
      toToken: TOKENS["pica"],
      price: 529.17,
      roi: 2.94,
      totalPurchased: "12,179,198.25	",
    },
    {
      token: TOKENS["pica"],
      toToken: TOKENS["ksm"],
      price: 529.17,
      roi: -12.3,
      totalPurchased: "12,179,198.25	",
    },
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
