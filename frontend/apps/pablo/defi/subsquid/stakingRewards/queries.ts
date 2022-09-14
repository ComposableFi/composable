import { fetchSubsquid } from "./helpers";

export interface StakingPosition {
    startTimestamp: string;
    fnftCollectionId: string;
    fnftInstanceId: string;
    endTimestamp: string;
    assetId: string;
    amount:string;
    owner: string;
    source: string;
    id: string;
}

export async function fetchStakingPositions(
    owner: string,
    orderBy: "ASC" | "DESC" = "DESC"
): Promise<Record<string, StakingPosition[]>> {
  let _stakingPositions : Record<string, StakingPosition[]> = {};
  try {
    const data = await fetchSubsquid<{  stakingPositions: StakingPosition[] }>
    (`query stakingPositions {
      stakingPositions (
        limit: 1,
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
      }, {} as Record<string, StakingPosition[]>);
    }

  } catch (error) {
    console.error(error);
  }

  return _stakingPositions;
}