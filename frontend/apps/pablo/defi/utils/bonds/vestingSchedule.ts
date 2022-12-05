import { SubsquidVestingScheduleEntity } from "@/defi/subsquid/bonds/queries";
import { VestingSchedule } from "@/defi/types";
import { BondedOfferVestingState } from "@/store/bond/bond.slice";
import { ApiPromise } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";
import { PALLET_TYPE_ID } from "../constants";
import { compareU8a, concatU8a } from "../misc";
import { fetchVestingSchedule } from "../vesting";
import { calculateClaimableAt } from "./vestingTime";
import { BondOffer } from "shared";
import BigNumber from "bignumber.js";

/**
 * get BondOfferId from VestingSchedule Account
 * returns -1 if invalid account
 * @param vestingScheduleSubAccount UInt8Array
 * @returns {BigNumber}
 */
export function getBondOfferIdByVestingScheduleAccount(
  parachainApi: ApiPromise,
  vestingScheduleSubAccount: Uint8Array
): BigNumber {
  let offerId = new BigNumber(-1);

  const bondedFiPalletId = concatU8a(
    PALLET_TYPE_ID,
    parachainApi.consts.bondedFinance.palletId.toU8a()
  );

  if (
    compareU8a(
      vestingScheduleSubAccount.subarray(0, bondedFiPalletId.length),
      bondedFiPalletId
    )
  ) {
    let paddedId = vestingScheduleSubAccount.subarray(
      bondedFiPalletId.length,
      vestingScheduleSubAccount.length
    );
    let firstNonZeroIndex = -1;

    for (let byteIndex = paddedId.length; byteIndex > 0; byteIndex--) {
      if (paddedId[byteIndex - 1] !== 0) {
        firstNonZeroIndex = byteIndex;
        byteIndex = 0;
      }
    }

    if (firstNonZeroIndex !== -1) {
      offerId = new BigNumber(
        u8aToHex(paddedId.subarray(0, firstNonZeroIndex))
      );
    }
  }

  return offerId;
}

/**
 * @param parachainApi ApiPromise
 * @param scheduleAddedEvents ScheduleAdded events form subsquid
 * @returns Record<string(bondOfferId), set<string>(vestingScheduleId)>
 */
export function createBondOfferIdVestingScheduleIdMap(
  parachainApi: ApiPromise,
  scheduleAddedEvents: SubsquidVestingScheduleEntity[]
): Record<string, Set<string>> {
  const scheduleFromBondedFi = scheduleAddedEvents
    .map((scheduleAddedEvent) => {
      const fromAccount = parachainApi
        .createType("AccountId32", scheduleAddedEvent.from)
        .toU8a();
      return {
        ...scheduleAddedEvent,
        bondOfferId: getBondOfferIdByVestingScheduleAccount(
          parachainApi,
          fromAccount
        ),
      };
    })
    .filter((scheduleAddedEvent) => !scheduleAddedEvent.bondOfferId.eq(-1));

  return scheduleFromBondedFi.reduce((acc, curr) => {
    let bondOfferId = curr.bondOfferId.toString();

    if (acc[bondOfferId]) {
      acc[bondOfferId].add(curr.scheduleId);
    } else {
      acc[bondOfferId] = new Set();
      acc[bondOfferId].add(curr.scheduleId);
    }

    return acc;
  }, {} as Record<string, Set<string>>);
}

export async function fetchVestingSchedulesByBondOffers(
  parachainApi: ApiPromise,
  account: string,
  bondOffers: BondOffer[],
  bondedOfferScheduleIds: Record<string, Set<string>>
): Promise<Record<string, VestingSchedule[]>> {
  let schedulesMap: Record<string, VestingSchedule[]> = {};

  bondOffers.forEach((offer) => {
    const offerId = offer.getBondOfferId() as string;
    schedulesMap[offerId] = [];
  });

  for (const offer of bondOffers) {
    const offerId = offer.getBondOfferId() as string;
    const schedules = await fetchVestingSchedule(
      parachainApi,
      account,
      offer.getRewardAssetId() as string
    );
    schedulesMap[offerId] = schedules.filter((schedule) =>
      bondedOfferScheduleIds[offerId]
        ? bondedOfferScheduleIds[offerId].has(
            schedule.vestingScheduleId.toString()
          )
        : false
    );
  }

  return schedulesMap;
}

export function calculateVestingState(
  blockNumber: BigNumber,
  blockInterval: BigNumber,
  bondedOfferSchedules: Record<string, VestingSchedule[]>
): Record<string, BondedOfferVestingState> {
  let bondedOfferVestingState = Object.keys(bondedOfferSchedules).reduce(
    (acc, c) => {
      const {
        pendingRewards,
        totalVested,
        alreadyClaimed,
        claimable
      } = calculateClaimableAt(bondedOfferSchedules[c][0], blockNumber);

      let milliSecondsSinceVestingStart = new BigNumber(0);
      if (bondedOfferSchedules[c].length > 0) {
        if(bondedOfferSchedules[c][0].window.start.lt(blockNumber)) {
          milliSecondsSinceVestingStart = blockInterval.times(
            blockNumber.minus(bondedOfferSchedules[c][0].window.start)
          )
        }
      }

      return {
        ...acc,
        [c]: {
          alreadyClaimed, 
          netRewards: totalVested,
          claimable,
          pendingRewards,
          milliSecondsSinceVestingStart: milliSecondsSinceVestingStart,
        } as BondedOfferVestingState
      };
    },
    {} as Record<string, BondedOfferVestingState>
  );

  return bondedOfferVestingState;
}
