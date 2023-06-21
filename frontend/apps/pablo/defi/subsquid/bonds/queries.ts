import { subsquidClient } from "@/defi/subsquid/client";
import { OperationResult } from "urql";

export const queryTotalPurchasedBondsByBondOfferIds = () => subsquidClient().query(`query queryTotalPurchasedBondsByBondOfferIds {
    bondedFinanceBondOffers {
      offerId
      totalPurchased
    }
}`).toPromise(); 

export interface SubsquidVestingScheduleEntity {
  scheduleId: string;
  id: string;
  from: string;
  eventId: string;
  to: string;
}

export function queryVestingSchedulesByAccountId(accountId: string): Promise<OperationResult<{
  vestingSchedules: SubsquidVestingScheduleEntity[]
}, {}>> {
  return subsquidClient().query(`
  query vestingSchedules {
    vestingSchedules(where: {to_eq: "${accountId}"}) {
      scheduleId
      id
      from
      eventId
      to
    }
  }  
  `).toPromise();
}
