import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import {
  usePicassoProvider,
  useSelectedAccount,
} from "@/defi/polkadot/hooks/index";
import { ApiPromise } from "@polkadot/api";
import { useStore } from "@/stores/root";
import { ActiveBond } from "@/stores/defi/polkadot/bonds/slice";
import { useQuery } from "@apollo/client";
import { GET_VESTING_SCHEDULE_BY_ADDRESS } from "@/apollo/queries/vestingSchedule";
import BigNumber from "bignumber.js";
import { stringToU8a, u8aToHex } from "@polkadot/util";
import { unwrapNumberOrHex } from "shared";

const PALLET_TYPE_ID = "modl";

export function concatU8a(a: Uint8Array, b: Uint8Array): Uint8Array {
  const c = new Uint8Array(a.length + b.length);
  c.set(a);
  c.set(b, a.length);
  return c;
}

export function compareU8a(a: Uint8Array, b: Uint8Array): boolean {
  if (a.length !== b.length) return false;

  let equal = true;

  a.forEach((a, i) => {
    if (a != b[i]) {
      equal = false;
    }
  });

  return equal;
}

export function getBondOfferIdByVestingScheduleAccount(
  parachainApi: ApiPromise,
  vestingScheduleSubAccount: Uint8Array
): BigNumber {
  let offerId = new BigNumber(-1);
  // @ts-ignore
  const bondedFiPalletId = concatU8a(
    stringToU8a(PALLET_TYPE_ID),
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

export function useActiveBonds() {
  const { bonds } = useStore<{
    bonds: BondOffer[];
    setActiveBonds: (itesm: ActiveBond[]) => void;
  }>((state) => ({
    bonds: state.bonds.bonds,
    setActiveBonds: state.bonds.setActiveBonds,
  }));
  const { parachainApi } = usePicassoProvider();
  const account = useSelectedAccount();
  const { data, loading, error } = useQuery(GET_VESTING_SCHEDULE_BY_ADDRESS, {
    variables: {
      accountId: account?.address,
    },
    pollInterval: 30000,
  });

  if (!loading && !error && data.vestingSchedules && parachainApi) {
    const result: Record<string, Set<string>> = data.vestingSchedules
      .map((vestingSchedule: any) => {
        const fromAccount = parachainApi
          ?.createType("AccountId32", vestingSchedule.from)
          .toU8a();
        return {
          ...vestingSchedule,
          bondOfferId: getBondOfferIdByVestingScheduleAccount(
            parachainApi,
            fromAccount
          ),
        };
      })
      .filter(
        (schedule: { bondOfferId: { toString: () => string } }) =>
          schedule.bondOfferId.toString() !== "-1"
      )
      .reduce(
        (
          acc: { [x: string]: { add: (arg0: any) => void } },
          curr: { bondOfferId: { toString: () => any }; id: any }
        ) => {
          let bondOfferId = curr.bondOfferId.toString();

          if (acc[bondOfferId]) {
            acc[bondOfferId].add(curr.id);
          } else {
            acc[bondOfferId] = new Set();
            acc[bondOfferId].add(curr.id);
          }

          return acc;
        },
        {} as Record<string, Set<string>>
      );

    const output: Array<ActiveBond> = [];
    for (const [bondOfferId, scheduleids] of Object.entries(result)) {
      scheduleids.forEach((scheduleId) => {
        const vestingSchedule = data.vestingSchedules.find(
          (schedule: any) => schedule.id.toString() === scheduleId.toString()
        );
        const bond = bonds.find(
          (bond) => bond.bondOfferId.toString() === bondOfferId.toString()
        );
        if (vestingSchedule && bond) {
          output.push({
            bond,
            periodCount: unwrapNumberOrHex(
              vestingSchedule.schedule.periodCount
            ),
            perPeriod: unwrapNumberOrHex(vestingSchedule.schedule.perPeriod),
            vestingScheduleId: vestingSchedule.id,
            window: {
              blockNumberBased: {
                start: unwrapNumberOrHex(vestingSchedule.schedule.window.start),
                period: unwrapNumberOrHex(
                  vestingSchedule.schedule.window.period
                ),
              },
            },
          });
        }
      });
    }
    return {
      loading,
      error,
      activeBonds: output,
    };
  }

  return {
    loading,
    error,
    activeBonds: [],
  };
}
