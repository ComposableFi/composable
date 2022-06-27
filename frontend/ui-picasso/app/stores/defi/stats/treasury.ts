import { NamedSet } from "zustand/middleware";
import { AllSlices, StoreSlice } from "../../types";
import StatsDummyData from "./dummyData";

export type TreasuryDataProps = {
  name: string;
  value: Array<number>;
  tooltip: string;
};

interface TreasuryData {
  data: Array<TreasuryDataProps>;
}

interface ChartData {
  name: string;
  value: number;
  change: number;
  data: Array<[number, number][]>;
}

interface TreasuryChartData {
  data: Array<{ data: ChartData }>;
}

interface TreasuryBondingData {
  bond: Array<{ label: string; description: string }>;
  claim: Array<{ label: string; description: string }>;
}

interface TreasuryState {
  treasuryData: TreasuryData;
  treasuryChartData: TreasuryChartData;
  treasuryBonding: TreasuryBondingData;
}

const initialState: TreasuryState = {
  treasuryData: {
    data: StatsDummyData.TREASURY.infoData,
  },
  treasuryChartData: {
    data: StatsDummyData.TREASURY.chartData,
  },
  treasuryBonding: {
    bond: StatsDummyData.TREASURY.bonding.bond,
    claim: StatsDummyData.TREASURY.bonding.claim,
  },
};

export interface StatsTreasurySlice {
  statsTreasury: TreasuryState & {
    setFeaturedMarketCap: (data: TreasuryData["data"][0]) => void;
    setFeaturedChaosPriceAndDiscount: (data: TreasuryData["data"][1]) => void;
    setFeaturedCirculatingSupply: (data: TreasuryData["data"][2]) => void;
    setFeaturedTreasuryBalance: (data: TreasuryData["data"][3]) => void;
    setFeaturedChaosApyAndRunway: (data: TreasuryData["data"][4]) => void;
    setFeaturedSchaos: (data: TreasuryData["data"][5]) => void;
    setChartMarketCap: (data: TreasuryChartData["data"][0]) => void;
    setChartTreasuryAssetValue: (data: TreasuryChartData["data"][1]) => void;
    setChaosStaked: (data: TreasuryChartData["data"][2]) => void;
    setTreasuryProportions: (data: TreasuryChartData["data"][3]) => void;
    setChartChaosApy: (data: TreasuryChartData["data"][4]) => void;
    setChartRevenue: (data: TreasuryChartData["data"][5]) => void;
    setChartBondProcess: (data: TreasuryChartData["data"][6]) => void;
    setChartTotalLiquidityOwned: (data: TreasuryChartData["data"][7]) => void;
    setTreasuryData: (data: TreasuryData) => void;
    setTreasuryChartData: (data: TreasuryChartData) => void;
  };
}

export const createStatsTreasurySlice: StoreSlice<StatsTreasurySlice> = (
  set: NamedSet<StatsTreasurySlice>
) => ({
  statsTreasury: {
    ...initialState,
    setFeaturedMarketCap: (data: TreasuryData["data"][0]) => {
      set((state: AllSlices) => {
        state.statsTreasury.treasuryData.data[0] = data;
      });
    },
    setFeaturedChaosPriceAndDiscount: (data: TreasuryData["data"][1]) => {
      set((state: AllSlices) => {
        state.statsTreasury.treasuryData.data[1] = data;
      });
    },
    setFeaturedCirculatingSupply: (data: TreasuryData["data"][2]) => {
      set((state: AllSlices) => {
        state.statsTreasury.treasuryData.data[2] = data;
      });
    },
    setFeaturedTreasuryBalance: (data: TreasuryData["data"][3]) => {
      set((state: AllSlices) => {
        state.statsTreasury.treasuryData.data[3] = data;
      });
    },
    setFeaturedChaosApyAndRunway: (data: TreasuryData["data"][4]) => {
      set((state: AllSlices) => {
        state.statsTreasury.treasuryData.data[4] = data;
      });
    },
    setFeaturedSchaos: (data: TreasuryData["data"][5]) => {
      set((state: AllSlices) => {
        state.statsTreasury.treasuryData.data[5] = data;
      });
    },
    setChartMarketCap: (data: TreasuryChartData["data"][0]) => {
      set((state: AllSlices) => {
        state.statsTreasury.treasuryChartData.data[0] = data;
      });
    },
    setChartTreasuryAssetValue: (data: TreasuryChartData["data"][1]) => {
      set((state: AllSlices) => {
        state.statsTreasury.treasuryChartData.data[1] = data;
      });
    },
    setChaosStaked: (data: TreasuryChartData["data"][2]) => {
      set((state: AllSlices) => {
        state.statsTreasury.treasuryChartData.data[2] = data;
      });
    },
    setTreasuryProportions: (data: TreasuryChartData["data"][3]) => {
      set((state: AllSlices) => {
        state.statsTreasury.treasuryChartData.data[3] = data;
      });
    },
    setChartChaosApy: (data: TreasuryChartData["data"][4]) => {
      set((state: AllSlices) => {
        state.statsTreasury.treasuryChartData.data[4] = data;
      });
    },
    setChartRevenue: (data: TreasuryChartData["data"][5]) => {
      set((state: AllSlices) => {
        state.statsTreasury.treasuryChartData.data[5] = data;
      });
    },
    setChartBondProcess: (data: TreasuryChartData["data"][6]) => {
      set((state: AllSlices) => {
        state.statsTreasury.treasuryChartData.data[6] = data;
      });
    },
    setChartTotalLiquidityOwned: (data: TreasuryChartData["data"][7]) => {
      set((state: AllSlices) => {
        state.statsTreasury.treasuryChartData.data[7] = data;
      });
    },
    setTreasuryData: (data: TreasuryData) => {
      set((state: AllSlices) => {
        state.statsTreasury.treasuryData = data;
      });
    },
    setTreasuryChartData: (data: TreasuryChartData) => {
      set((state: AllSlices) => {
        state.statsTreasury.treasuryChartData = data;
      });
    },
  },
});
