import { SubsquidVestingScheduleEntity } from "@/defi/subsquid/bonds/queries";
import { VestingSchedule } from "@/defi/types";
import { ApiPromise } from "@polkadot/api";
import { u8aToHex, stringToU8a } from "@polkadot/util";
import BigNumber from "bignumber.js";
import { PALLET_TYPE_ID } from "../constants";
import { compareU8a, concatU8a } from "../misc";

/**
 * get BondOfferId from VestingSchedule Account
 * returns -1 if invalid account
 * @param vestingScheduleAccount UInt8Array
 * @returns BigNumber
 */
export function getBondOfferIdByVestingScheduleAccount(
  parachainApi: ApiPromise,
  vestingScheduleAccount: Uint8Array
): BigNumber {
  let offerId = new BigNumber(-1);
  // @ts-ignore
  const bondedFiPalletId = concatU8a(
    stringToU8a(PALLET_TYPE_ID),
    parachainApi.consts.bondedFinance.palletId.toU8a()
  );

  if (
    compareU8a(
      vestingScheduleAccount.subarray(0, bondedFiPalletId.length),
      bondedFiPalletId
    )
  ) {
    let paddedId = vestingScheduleAccount.subarray(
      bondedFiPalletId.length,
      vestingScheduleAccount.length
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
      acc[bondOfferId].add(curr.vestingScheduleId);
    } else {
      acc[bondOfferId] = new Set();
      acc[bondOfferId].add(curr.vestingScheduleId);
    }

    return acc;
  }, {} as Record<string, Set<string>>);
}

export function calculateClaimableAt(
  vestingSchedule: VestingSchedule | undefined,
  currentBlock: BigNumber
): {
  claimable: BigNumber;
  pendingRewards: BigNumber;
} {
  let claimable = new BigNumber(0),
    pendingRewardsToBeClaimed = new BigNumber(0);

  if (vestingSchedule) {
    if (vestingSchedule.type === "block") {
      const { start, period } = vestingSchedule.window;
      const { perPeriod, alreadyClaimed, periodCount } = vestingSchedule;

      let totalClaimable = perPeriod.times(periodCount);
      pendingRewardsToBeClaimed = totalClaimable.minus(alreadyClaimed);

      if (currentBlock.gt(start.plus(period.times(periodCount)))) {
        claimable = pendingRewardsToBeClaimed.gt(perPeriod)
          ? perPeriod
          : pendingRewardsToBeClaimed;
      } else {
        let startBlock = new BigNumber(start);
        let rewardedAmount = new BigNumber(0);
        while (startBlock.lt(currentBlock)) {
          rewardedAmount = rewardedAmount.plus(perPeriod);
          startBlock = startBlock.plus(period);
        }

        claimable = rewardedAmount.minus(alreadyClaimed);
      }
    }
  }

  return {
    claimable,
    pendingRewards: pendingRewardsToBeClaimed,
  };
}
