import { fetchSubsquid } from "../stakingRewards/helpers";

type Currency = "USD";
enum LockedSource {
    All = "All",
    Pablo = "Pablo",
    StakingRewards = "StakingRewards",
    VestingSchedules = "VestingSchedules"
}

export type HistoricalLockedValue = {
    amount?: string;
    currency?: Currency;
    source?: LockedSource;
    id: string;
    timestamp?: string;
}

type HistoricalLockedValues = {
    historicalLockedValues: Array<Required<HistoricalLockedValue>>
}

const queryHistoricalLockedValues = (source: LockedSource = LockedSource.Pablo) => `
query historicalLockedValues {
    historicalLockedValues (where: { source_eq: ${source} }) {
      amount
      currency
      id
      source
      timestamp
    }
}  
`

export async function pabloHistoricalValues(): Promise<Array<Required<HistoricalLockedValue>>> {
    try {
        const { historicalLockedValues } = await fetchSubsquid<Required<HistoricalLockedValues>>(queryHistoricalLockedValues(), true);

        return historicalLockedValues;
    } catch (err: any) {
        console.error('[pabloHistoricalValues] ', err);
        return [];
    }
}