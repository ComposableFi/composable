import {
  queryTotalPurchasedBondsByBondOfferIds,
  queryVestingSchedulesByAccountId,
  SubsquidVestingScheduleEntity,
} from "./queries";
import BigNumber from "bignumber.js";
import { ApiPromise } from "@polkadot/api";
import { stringToBytes } from "micro-base";
import { u8aToHex } from "@polkadot/util";
import { compareU8a, fetchVestingSchedule } from "@/defi/utils";
import { BondOffer } from "@/defi/types";

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

export async function fetchBondVestingSchedules(
  parachainApi: ApiPromise,
  bondOffers: BondOffer[],
  accountId: string
): Promise<Array<BondOffer>> {
  let offers: BondOffer[] = [];
  try {
    const { data, error } = await queryVestingSchedulesByAccountId(accountId);
    if (error) throw new Error(error.message);
    if (!data)
    throw new Error("fetchVestingSchedulesByAccount: Data unavailable.");
    let { vestingSchedules } = data;
    const bondedFiPalletId = stringToBytes("utf8", "modlbondedfi");

    const mapped = vestingSchedules.map((schedule) => {
      const accountId32 = parachainApi
        .createType("AccountId32", schedule.from)
        .toU8a();

      if (
        compareU8a(
          accountId32.subarray(0, bondedFiPalletId.length),
          bondedFiPalletId
        )
      ) {
        let subArr = accountId32.subarray(
          bondedFiPalletId.length,
          accountId32.length
        );
        let firstNonZeroIndex = -1;

        for (let i = subArr.length; i > 0; i--) {
          if (subArr[i - 1] !== 0) {
            firstNonZeroIndex = i;
            i = -1;
          }
        }

        if (firstNonZeroIndex !== -1) {
          const offerId = new BigNumber(
            u8aToHex(subArr.subarray(0, firstNonZeroIndex))
          );
          return { ...schedule, bondOfferId: offerId };
        }
      }
      return undefined;
    });

    const filteredSchedules = mapped.filter(i => i !== undefined);

    offers = bondOffers.filter(i => {
      return filteredSchedules.some(fs => fs?.bondOfferId.eq(i.offerId))
    });

    const vs = await Promise.all(offers.map(i => fetchVestingSchedule(parachainApi, accountId, i.reward.asset)));

    console.log('vs', vs);

  } catch (err) {
    console.error(err);
  } finally {
    return offers;
  }
}
