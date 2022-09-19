import { createSlice } from "@reduxjs/toolkit";
import BigNumber from "bignumber.js";

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

export type BondChartData = {
  total: BigNumber,
  change: number,
  series: [number, number][],
};

interface PolkadotState {
  overview: Overview;
  stakingOverview: StakingOverview,
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
  bondPortfolioChartData: {
    total: new BigNumber(24546395.04),
    change: 2,
    series: [],
  }
};

export const polkadotSlice = createSlice({
  name: "PolkaDot",
  initialState,
  reducers: {},
});

export default polkadotSlice.reducer;
