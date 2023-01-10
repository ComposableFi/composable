import { fetchSubsquid } from "../stakingRewards/helpers";
import { tryCatch } from "fp-ts/TaskEither";

type Currency = "USD";

enum LockedSource {
  All = "All",
  Pablo = "Pablo",
  StakingRewards = "StakingRewards",
  VestingSchedules = "VestingSchedules",
}

export type HistoricalLockedValue = {
  amount?: string;
  currency?: Currency;
  source?: LockedSource;
  id: string;
  timestamp?: string;
};

type HistoricalLockedValues = {
  historicalLockedValues: Array<Required<HistoricalLockedValue>>;
};

const queryHistoricalLockedValues = (
  source: LockedSource = LockedSource.Pablo
) => `
query historicalLockedValues {
    historicalLockedValues (where: { source_eq: ${source} }) {
      amount
      currency
      id
      source
      timestamp
    }
}  
`;

export async function pabloHistoricalValues(): Promise<
  Array<Required<HistoricalLockedValue>>
> {
  try {
    const { historicalLockedValues } = await fetchSubsquid<
      Required<HistoricalLockedValues>
    >(queryHistoricalLockedValues(), true);

    return historicalLockedValues;
  } catch (err: any) {
    console.error("[pabloHistoricalValues] ", err);
    return [];
  }
}

type AssetAmount = {
  assetId: string;
  amount: string;
};

type PabloOverviewStats = {
  totalValueLocked: AssetAmount[];
};

export function fetchPabloOverviewStatsTVL() {
  return tryCatch(
    async () =>
      await fetchSubsquid<{
        pabloOverviewStats: PabloOverviewStats;
      }>(queryPabloOverviewStatsTVL(), true),
    () => ({ pabloOverviewStats: { totalValueLocked: [] } })
  );
}

const queryPabloOverviewStatsTVL = () => `
query pabloOverviewStatsTVL {
  pabloOverviewStats {
    totalValueLocked {
      amount
      assetId
    }
  }
}
`;

type PabloOverviewTVLChart = {
  totalValueLocked: {
    lockedValues: {
      amount: string;
      assetId: string;
    }[];
    date: string;
  }[];
};

const range = ["day", "week", "month", "year"] as const;

export type Range = typeof range[number];

export function fetchPabloOverviewTVLChart(range: Range) {
  return () =>
    tryCatch(
      async () =>
        await fetchSubsquid<PabloOverviewTVLChart>(
          queryPabloOverviewTVLChart(range),
          true
        ),
      () =>
        ({
          totalValueLocked: [],
        } as PabloOverviewTVLChart)
    );
}

const queryPabloOverviewTVLChart = (range: Range) => `
query totalValueLockedChart {
  totalValueLocked(params: {range: "${range}", source: "Pablo"}) {
    lockedValues {
      amount
      assetId
    }
    date
  }
}
`;

type PabloOverviewDailyVolume = {
  pabloOverviewStats: {
    dailyVolume: Array<{
      assetId: string;
      amount: string;
    }>;
  };
};

export const queryPabloOverviewDailyVolume = () => `
query MyQuery {
  pabloOverviewStats {
    dailyVolume {
      amount
      assetId
    }
  }
} 
`;

export function fetchPabloOverviewDailyVolume() {
  return tryCatch(
    async () =>
      await fetchSubsquid<PabloOverviewDailyVolume>(
        queryPabloOverviewDailyVolume(),
        true
      ),
    () =>
      ({
        pabloOverviewStats: {
          dailyVolume: [],
        },
      } as PabloOverviewDailyVolume)
  );
}

type PabloTotalVolume = {
  pabloTotalVolume: Array<{
    date: string;
    volumes: Array<{
      assetId: string;
      amount: string;
    }>;
  }>;
};

export const queryPabloTotalVolume = (range: Range) => `
query MyQuery {
  pabloTotalVolume(params: {range: "${range}"}) {
    date
    volumes {
      amount
      assetId
    }
  }
}`;

export function fetchPabloTotalVolume(range: Range) {
  return () =>
    tryCatch(
      async () =>
        await fetchSubsquid<PabloTotalVolume>(
          queryPabloTotalVolume(range),
          true
        ),
      () =>
        ({
          pabloTotalVolume: [],
        } as PabloTotalVolume)
    );
}
