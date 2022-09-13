import { OperationResult } from "urql";
import { makeClient } from "../makeClient";

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

export function queryStakingPositions(
    owner: string,
    principalAssetId: string,
    orderBy: "ASC" | "DESC" = "DESC"
): Promise<OperationResult<{
  stakingPositions: StakingPosition[]
}, {}>> {
  return makeClient().query(`
    query stakingPositions {
      stakingPositions (
        limit: 1,
        where: {
            assetId_eq: "${principalAssetId}",
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
    }
  `).toPromise();
}