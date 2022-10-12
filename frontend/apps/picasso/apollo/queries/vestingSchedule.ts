import { gql } from "@apollo/client";

export type VestingSchedule = {
  id: string;
  from: string;
  eventId: string;
  scheduleId: string;
  to: string;
};
export type VestingSchedules = {
  vestingSchedules: VestingSchedule[];
};
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
