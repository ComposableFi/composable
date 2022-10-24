import { gql } from "@apollo/client";

export type TVL = {
  totalValueLocked: number,
  date: string
}

export type TotalValueLocked = {
  totalValueLocked: TVL[]
}

export const GET_TOTAL_VALUE_LOCKED = gql`
    query getTotalValueLocked($interval: Int!, $dateTo: String!, $dateFrom: String ) {
        totalValueLocked(params: {intervalMinutes: $interval, dateTo: $dateTo, dateFrom: $dateFrom}) {
            date
            totalValueLocked
        }
    }
`;
