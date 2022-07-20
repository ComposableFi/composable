import { queryTotalPurchasedBondsByBondOfferIds, queryVestingSchedulesByAccountId, SubsquidVestingScheduleEntity } from "./queries";
import BigNumber from "bignumber.js";
import { ApiPromise } from "@polkadot/api";
import { stringToBytes } from "micro-base";
import { u8aToHex } from "@polkadot/util";

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

export async function fetchBondVestingSchedules(parachainApi: ApiPromise, accountId: string): Promise<SubsquidVestingScheduleEntity[]> {
  let schedules: SubsquidVestingScheduleEntity[] = [];
  try {
    const { data, error } = await queryVestingSchedulesByAccountId(accountId);
    if (error) throw new Error(error.message);
    if (!data) throw new Error('fetchVestingSchedulesByAccount: Data unavailable.');
    let { vestingSchedules } = data;

    vestingSchedules = vestingSchedules.filter(schedule => {
      const accountId32 = parachainApi.createType("AccountId32", schedule.from);
      const bondedFiPalletId = u8aToHex(stringToBytes("utf8", "modlbondedfi"))

      if (accountId32.toHex().startsWith(bondedFiPalletId)) {
        return true;
      }
      return false;
    });

    schedules = vestingSchedules;
  } catch (err) {
    console.error(err);
  } finally {
    return schedules;
  }
}