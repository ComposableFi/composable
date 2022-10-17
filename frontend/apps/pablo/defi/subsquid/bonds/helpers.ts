import {
  queryTotalPurchasedBondsByBondOfferIds,
  queryVestingSchedulesByAccountId,
  SubsquidVestingScheduleEntity,
} from "./queries";
import BigNumber from "bignumber.js";
import { ApiPromise } from "@polkadot/api";
import { createBondOfferIdVestingScheduleIdMap } from "@/defi/utils";

export async function fetchTotalPurchasedBondsByOfferIds(): Promise<
  Record<string, BigNumber>
> {
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
        c: { offerId: string; totalPurchased: string }
      ) => {
        return {
          ...p,
          [c.offerId]: new BigNumber(c.totalPurchased),
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

export async function fetchVestingSchedulesAdded(
  accountId: string
): Promise<SubsquidVestingScheduleEntity[]> {
  let schedulesAdded: SubsquidVestingScheduleEntity[] = [];
  try {
    const { data, error } = await queryVestingSchedulesByAccountId(accountId);
    if (error) throw new Error(error.message);
    if (!data)
      throw new Error("fetchVestingSchedulesByAccount: Data unavailable.");
    let { vestingSchedules } = data;

    schedulesAdded = vestingSchedules;
  } catch (err) {
    console.error(err);
  } finally {
    return schedulesAdded;
  }
}

export async function extractUserBondedFinanceVestingScheduleAddedEvents(
  parachainApi: ApiPromise,
  userAccount: string
): Promise<Record<string, Set<string>>> {
  let bondedOfferIdVestingScheduleIdRecord = {};

  try {
    const scheduleAddedEvents = await fetchVestingSchedulesAdded(userAccount);
    bondedOfferIdVestingScheduleIdRecord =
      createBondOfferIdVestingScheduleIdMap(parachainApi, scheduleAddedEvents);
  } catch (err: any) {
    console.error(err);
  }

  return bondedOfferIdVestingScheduleIdRecord;
}
