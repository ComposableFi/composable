import { gql } from "@apollo/client";

export type TVL = {
  totalValueLocked: number,
  date: string
}

export type TotalValueLocked = {
  totalValueLocked: TVL[]
}

export const GET_TOTAL_VALUE_LOCKED = gql`
    query getTotalValueLocked($range: String! ) {
        totalValueLocked(params: {range: $range}) {
            date
            totalValueLocked
        }
    }
`;
