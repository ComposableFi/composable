import { queryTotalPurchasedBondsByBondOfferIds, queryVestingSchedulesByAccountId } from "./queries";
import BigNumber from "bignumber.js";

export async function fetchTotalPurchasedBondsByOfferIds(): Promise<Record<string, BigNumber>> {
  let totalPurchasedMap: Record<string, BigNumber> = {};
  try {
    let { data, error } = await queryTotalPurchasedBondsByBondOfferIds();
    if (!data)
      throw new Error(
        `fetchTotalPurchasedBondsByOfferIds unable to fetch subsquid data`
      );
    if (error) throw new Error(error.message);

    let { bondedFinanceBondOffers } = data;

    totalPurchasedMap = bondedFinanceBondOffers.reduce(
      (
        p: Record<string, BigNumber>,
        c: { id: string; totalPurchased: string }
      ) => {
        return {
          ...p,
          [c.id]: new BigNumber(c.totalPurchased),
        };
      },
      {} as Record<string, BigNumber>
    );
  } catch (err) {
    console.error(err);
  } finally {
    return totalPurchasedMap;
  }
}

interface SubsquidVestingScheduleEntity {
  scheduleId: string;
  id: string;
  from: string;
  eventId: string;
  beneficiary: string;
}

export async function fetchVestingSchedulesByAccount(accountId: string): Promise<SubsquidVestingScheduleEntity[]> {
  let schedules: SubsquidVestingScheduleEntity[] = [];
  try {
    const { data, error } = await queryVestingSchedulesByAccountId(accountId);
    if (error) throw new Error(error.message);
    if (!data) throw new Error('fetchVestingSchedulesByAccount: Data unavailable.');
    const { vestingSchedules } = data;

    schedules = vestingSchedules;
  } catch (err) {
    console.error(err);
  } finally {
    return schedules;
  }
}