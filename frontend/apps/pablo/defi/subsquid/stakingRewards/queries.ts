import { StakingPositionHistory } from "@/defi/types";
import { fetchSubsquid } from "./helpers";

export const PABLO_STAKING_OVERVIEW_QUERY = `
  query pabloStakingOverviewQuery {
    pabloOverviewStats {
      averageLockMultiplier
      averageLockTime
      totalValueLocked
    }
  }
`;

export async function fetchStakingPositionHistory(
    owner: string,
    orderBy: "ASC" | "DESC" = "DESC"
): Promise<Record<string, StakingPositionHistory[]>> {
  let _stakingPositions : Record<string, StakingPositionHistory[]> = {};
  try {
    const data = await fetchSubsquid<{  stakingPositions: StakingPositionHistory[] }>
    (`query stakingPositions {
      stakingPositions (
        where: {
            owner_eq: "${owner}"
        },
        orderBy: startTimestamp_${orderBy}
      ) {
        startTimestamp
        owner
        source
        id
        fnftCollectionId
        fnftInstanceId
        endTimestamp
        assetId
        amount
      }
    }`);
  
    const { stakingPositions } = data;
    if (stakingPositions.length > 0) {
      _stakingPositions = stakingPositions.reduce((agg, curr) => {
        let currAssetId = curr.assetId;
        if (agg[currAssetId]) {
          agg[currAssetId].push(curr);
        } else {
          agg[currAssetId] = [curr]
        }
        return agg;
      }, {} as Record<string, StakingPositionHistory[]>);
    }

  } catch (error) {
    console.error(error);
  }

  return _stakingPositions;
}