import { gql } from "@apollo/client";

export interface StakingPosition {
  amount: BigInt;
  assetId: string;
  endTimestamp: BigInt;
  fnftCollectionId: string;
  fnftInstanceId: string;
  id: string;
  owner: string;
  source: "StakingRewards";
  startTimestamp: BigInt;
}

export interface StakingPositions {
  stakingPositions: StakingPosition[];
}

export const GET_STAKING_POSITIONS = gql`
    query stakingPositionsByOwner($accountId: String) {
        stakingPositions(orderBy: fnftCollectionId_ASC, where: {owner_eq: $accountId}) {
            amount
            assetId
            endTimestamp
            fnftCollectionId
            fnftInstanceId
            id
            owner
            source
            startTimestamp
        }
    }

`;

export type TotalValueLocked = {
  totalValueLocked: {
    date: string;
    totalValueLocked: BigInt
  }
}

export const GET_TOTAL_VALUE_LOCKED = gql`
    query totalValueLocked($dateString: String) {
        totalValueLocked(params: {intervalMinutes: 10, dateTo: $dateString}) {
            date
            totalValueLocked
        }
    }
`;
