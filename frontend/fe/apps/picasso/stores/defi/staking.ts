import { createAsyncThunk, createSlice, PayloadAction } from "@reduxjs/toolkit";
import { formatNumber } from "@/utils/formatters";
import BigNumber from "bignumber.js";

export const DURATION_OPTIONS = {
  "2w": "2 weeks",
  "2m": "2 months",
  "1y": "1 year",
  "2y": "2 years",
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
};

export type OpenPositions = Array<OpenPosition>;

export const renewPeriod = createAsyncThunk(
  "staking/renewPeriod",
  async (extendPeriod: string, thunkAPI) => {
    return extendPeriod;
  }
);

export const burnUnstake = createAsyncThunk("staking/burnUnstake", async () => {
  return "";
});

export const initialState = {
  highlights: {
    totalPicaLocked: formatNumber(20325651),
    totalChaosAPY: "265%",
    totalChaosMinted: formatNumber(4265),
    averageLockMultiplier: formatNumber(0.8),
    averageLockTime: "265 days",
  },
  claimableRewards: {
    pica: new BigNumber(25.135),
    pablo: new BigNumber(65.265),
    angl: new BigNumber(48.551),
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
      usdValue: new BigNumber(34567),
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
      usdValue: new BigNumber(-34567),
    },
  ],
  initialPicaDeposit: new BigNumber(55655),
  withdrawablePica: new BigNumber(25565),
};

export const stakingSlice = createSlice({
  name: "Staking",
  initialState,
  reducers: {
    setStakingHighlights: (
      state,
      action: PayloadAction<StakingHighlightsType>
    ) => {
      state.highlights = action.payload;
    },
    setClaimableRewards: (state, action: PayloadAction<ClaimableRewards>) => {
      state.claimableRewards = action.payload;
    },
  },
  extraReducers: (builder) => {
    builder.addCase(renewPeriod.fulfilled, (state, action) => {
      // TOOD: update open positions
    });
    builder.addCase(burnUnstake.fulfilled, (state, action) => {
      // TODO: update deposit and withdrawable pica
    });
  },
});

export const { setStakingHighlights, setClaimableRewards } =
  stakingSlice.actions;

export default stakingSlice.reducer;
