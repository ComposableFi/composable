import { gql } from "@apollo/client";

export const GET_VESTING_SCHEDULE_BY_ADDRESS = gql`
  query VestingScheduleByAccountId($accountId: String) {
    vestingSchedules(where: { to_eq: $accountId }) {
      id
      from
      eventId
      scheduleId
      to
    }
  }
`;
