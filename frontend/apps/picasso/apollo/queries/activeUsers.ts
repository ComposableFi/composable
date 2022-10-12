import { gql } from "@apollo/client";

export type ActiveUser = {
  count: number,
  date: string
}

export type ActiveUsers = {
  activeUsers: ActiveUser[]
}

export const GET_ACTIVE_USERS = gql`
    query getActiveUsersQuery($interval: Int!, $dateTo: String!, $dateFrom: String ) {
        activeUsers(params: {intervalMinutes: $interval, dateTo: $dateTo, dateFrom: $dateFrom}) {
            count
            date
        }
    }
`;
