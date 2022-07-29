import { NamedSet } from "zustand/middleware";
import { StoreSlice } from "../types";
import { formatNumber } from "shared";
import BigNumber from "bignumber.js";

export const DURATION_OPTIONS = {
  "2w": "2 weeks",
  "2m": "2 months",
  "1y": "1 year",
  "2y": "2 years"
};

export type DurationOption = keyof typeof DURATION_OPTIONS;

export interface StakingState {
  highlights: StakingHighlightsType;
  openPositions: OpenPositions;
  claimableRewards: ClaimableRewards;
  initialPicaDeposit: BigNumber;
  withdrawablePica: BigNumber;
}

export type StakingHighlightsType = {
  totalPicaLocked: string;
  totalChaosAPY: string;
  totalChaosMinted: string;
  averageLockMultiplier: string;
  averageLockTime: string;
};

export type ClaimableRewards = {
  pica: BigNumber;
  pablo: BigNumber;
  angl: BigNumber;
};

export type OpenPosition = {
  id: string;
  lockedPica: BigNumber;
  expiryDate: number;
  multiplier: number;
  yourChaos: BigNumber;
  value: BigNumber;
  usdValue: BigNumber;
};

export type OpenPositions = Array<OpenPosition>;

export const renewPeriod = async (extendPeriod: string) => {
  return extendPeriod;
};

export const burnUnstake = async () => {
  return "";
};

export const initialState = {
  highlights: {
    totalPicaLocked: formatNumber(20325651),
    totalChaosAPY: "265%",
    totalChaosMinted: formatNumber(4265),
    averageLockMultiplier: formatNumber(0.8),
    averageLockTime: "265 days"
  },
  claimableRewards: {
    pica: new BigNumber(25.135),
    pablo: new BigNumber(65.265),
    angl: new BigNumber(48.551)
  },
  openPositions: [
    {
      id: "FNFT 236",
      lockedPica: new BigNumber(0),
      expiryDate: Math.round(
        new Date(Date.now() + 1000 * 60 * 60 * 24 * 7).getTime() / 1000
      ),
      multiplier: 1.0,
      yourChaos: new BigNumber(0),
      value: new BigNumber(23.309),
      usdValue: new BigNumber(34567)
    },
    {
      id: "FNFT 234",
      lockedPica: new BigNumber(0),
      expiryDate: Math.round(
        new Date(Date.now() + 1000 * 60 * 60 * 24 * 7).getTime() / 1000
      ),
      multiplier: 1.0,
      yourChaos: new BigNumber(0),
      value: new BigNumber(23.309),
      usdValue: new BigNumber(-34567)
    }
  ],
  initialPicaDeposit: new BigNumber(55655),
  withdrawablePica: new BigNumber(25565)
};

export interface StakingSlice {
  staking: StakingState & {
    setStakingHighlights: (data: StakingHighlightsType) => void;
    setClaimableRewards: (data: ClaimableRewards) => void;
  };
}

export const createStakingSlice: StoreSlice<StakingSlice> = (
  set: NamedSet<StakingSlice>
) => ({
  staking: {
    ...initialState,

    setStakingHighlights: (data: StakingHighlightsType) => {
      set(state => {
        state.staking.highlights = data;

        return state;
      });
    },
    setClaimableRewards: (data: ClaimableRewards) => {
      set(state => {
        state.staking.claimableRewards = data;

        return state;
      });
    }
  }
});
