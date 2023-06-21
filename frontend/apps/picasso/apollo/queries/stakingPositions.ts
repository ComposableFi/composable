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
