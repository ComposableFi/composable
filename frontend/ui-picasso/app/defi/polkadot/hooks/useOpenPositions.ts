import { useAppSelector } from "@/hooks/store";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { useEffect, useMemo } from "react";
import { usePicassoProvider } from "@/defi/polkadot/hooks/index";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";

export interface VestingSchedule {
  window: {
    blockNumberBased: {
      start: BigNumber;
      period: BigNumber;
    };
  };
  periodCount: BigNumber;
  perPeriod: BigNumber;
}
type VestingAccount = { name: string; address: string };

export function useOpenPositions(account: VestingAccount | undefined) {
  const bonds = useAppSelector<BondOffer[]>((state) => state.bonding.bonds);
  const { parachainApi } = usePicassoProvider();

  // traverse to all bonds
  // get all reward assets
  // fetch each reward asset and create a new array

  const rewardIds = useMemo(
    () =>
      bonds.reduce((acc, cur) => {
        acc.push(cur.reward.assetId);
        return acc;
      }, <string[]>[]),
    bonds
  );

  function unwrapNumberOrHex(v: string | number) {
    return new BigNumber(v, v.toString().startsWith("0x") ? 16 : 10);
  }
  async function subscribeVestingSchedule(
    api: ApiPromise,
    acc: VestingAccount
  ) {
    const unsub = await api.query.vesting.vestingSchedules.multi(
      rewardIds.map((id) => [acc.address, id]),
      (schedules) => {
        const out = schedules.flatMap((schedule) => {
          return schedule.flatMap((each) => each.toJSON());
        });
        const state: VestingSchedule[] = out
          .map((v) => {
            if (v) {
              const perPeriod = unwrapNumberOrHex(v.perPeriod);
              const periodCount = unwrapNumberOrHex(v.periodCount);
              const window = {
                blockNumberBased: {
                  start: unwrapNumberOrHex(v.window.blockNumberBased.start),
                  periodCount: unwrapNumberOrHex(
                    v.window.blockNumberBased.period
                  ),
                },
              };
              return {
                perPeriod,
                periodCount,
                window,
              };
            } else {
              return null;
            }
          })
          .filter(Boolean);
        console.log("State", state);
      }
    );
    return () => unsub();
  }

  useEffect(() => {
    let unsub = () => {};
    if (parachainApi && account) {
      subscribeVestingSchedule(parachainApi, account).then(
        (uns) => (unsub = uns)
      );
    }

    return () => unsub();
  }, [parachainApi, bonds, account]);
}
