import { NamedSet } from "zustand/middleware";
import { StoreSlice } from "../../types";
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
      set((state) => {
        state.statsTreasury.treasuryData.data[0] = data;

        return state;
      });
    },
    setFeaturedChaosPriceAndDiscount: (data: TreasuryData["data"][1]) => {
      set((state) => {
        state.statsTreasury.treasuryData.data[1] = data;

        return state;
      });
    },
    setFeaturedCirculatingSupply: (data: TreasuryData["data"][2]) => {
      set((state) => {
        state.statsTreasury.treasuryData.data[2] = data;

        return state;
      });
    },
    setFeaturedTreasuryBalance: (data: TreasuryData["data"][3]) => {
      set((state) => {
        state.statsTreasury.treasuryData.data[3] = data;

        return state;
      });
    },
    setFeaturedChaosApyAndRunway: (data: TreasuryData["data"][4]) => {
      set((state) => {
        state.statsTreasury.treasuryData.data[4] = data;

        return state;
      });
    },
    setFeaturedSchaos: (data: TreasuryData["data"][5]) => {
      set((state) => {
        state.statsTreasury.treasuryData.data[5] = data;

        return state;
      });
    },
    setChartMarketCap: (data: TreasuryChartData["data"][0]) => {
      set((state) => {
        state.statsTreasury.treasuryChartData.data[0] = data;

        return state;
      });
    },
    setChartTreasuryAssetValue: (data: TreasuryChartData["data"][1]) => {
      set((state) => {
        state.statsTreasury.treasuryChartData.data[1] = data;

        return state;
      });
    },
    setChaosStaked: (data: TreasuryChartData["data"][2]) => {
      set((state) => {
        state.statsTreasury.treasuryChartData.data[2] = data;

        return state;
      });
    },
    setTreasuryProportions: (data: TreasuryChartData["data"][3]) => {
      set((state) => {
        state.statsTreasury.treasuryChartData.data[3] = data;

        return state;
      });
    },
    setChartChaosApy: (data: TreasuryChartData["data"][4]) => {
      set((state) => {
        state.statsTreasury.treasuryChartData.data[4] = data;

        return state;
      });
    },
    setChartRevenue: (data: TreasuryChartData["data"][5]) => {
      set((state) => {
        state.statsTreasury.treasuryChartData.data[5] = data;

        return state;
      });
    },
    setChartBondProcess: (data: TreasuryChartData["data"][6]) => {
      set((state) => {
        state.statsTreasury.treasuryChartData.data[6] = data;

        return state;
      });
    },
    setChartTotalLiquidityOwned: (data: TreasuryChartData["data"][7]) => {
      set((state) => {
        state.statsTreasury.treasuryChartData.data[7] = data;

        return state;
      });
    },
    setTreasuryData: (data: TreasuryData) => {
      set((state) => {
        state.statsTreasury.treasuryData = data;

        return state;
      });
    },
    setTreasuryChartData: (data: TreasuryChartData) => {
      set((state) => {
        state.statsTreasury.treasuryChartData = data;

        return state;
      });
    },
  },
});
