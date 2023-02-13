import { gql } from "@apollo/client";

export type ActiveUser = {
  count: number,
  date: string
}

export type ActiveUsers = {
  activeUsers: ActiveUser[]
}

export const GET_ACTIVE_USERS = gql`
    query getActiveUsersQuery($range: String! ) {
        activeUsers(params: {range: $range}) {
            count
            date
        }
    }
`;
