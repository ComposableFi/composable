import { useAppDispatch, useAppSelector } from "@/hooks/store";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { useEffect } from "react";
import { usePicassoProvider } from "@/defi/polkadot/hooks/index";
import { ApiPromise } from "@polkadot/api";
import { updateOpenPositions } from "@/stores/defi/polkadot/bonds/slice";
import { unwrapNumberOrHex } from "@/utils/hexStrings";

type VestingAccount = { name: string; address: string };

const bondedVestingSchedule =
  (bond: BondOffer) => (address: string) => (api: ApiPromise) => {
    return async () => {
      const vestingScheduleResponse = await api.query.vesting.vestingSchedules(
        address,
        bond.reward.assetId
      );

      return vestingScheduleResponse
        ? vestingScheduleResponse.flatMap((vs) => {
            const jsonVestingSchedule: any = vs?.toJSON() ?? null;
            if (jsonVestingSchedule) {
              const perPeriod = unwrapNumberOrHex(
                (jsonVestingSchedule as any).perPeriod
              );
              const periodCount = unwrapNumberOrHex(
                (jsonVestingSchedule as any).periodCount
              );
              const window = {
                blockNumberBased: {
                  start: unwrapNumberOrHex(
                    (jsonVestingSchedule as any).window.blockNumberBased.start
                  ),
                  period: unwrapNumberOrHex(
                    (jsonVestingSchedule as any).window.blockNumberBased.period
                  ),
                },
              };
              return {
                bond,
                perPeriod,
                periodCount,
                window,
                type: window.blockNumberBased ? "block" : "time",
              };
            }
            return null;
          })
        : [];
    };
  };

export function useOpenPositions(account: VestingAccount | undefined) {
  const bonds = useAppSelector<BondOffer[]>((state) => state.bonding.bonds);
  const { parachainApi } = usePicassoProvider();
  const dispatch = useAppDispatch();

  // traverse to all bonds
  // get all reward assets
  // fetch each reward asset and create a new ar
  async function fetchVestingSchedules(api: ApiPromise, acc: VestingAccount) {
    const allVesting = bonds
      .flatMap(
        async (bond) => await bondedVestingSchedule(bond)(acc.address)(api)()
      )
      .flat();
    return Promise.all(allVesting);
  }

  async function fetchAndStore(factoryFn: () => Promise<unknown>) {
    const result: any = await factoryFn();
    dispatch(updateOpenPositions(result.filter(Boolean).flat()));
  }

  useEffect(() => {
    let unsub = () => {};
    if (parachainApi && account) {
      fetchAndStore(() => fetchVestingSchedules(parachainApi, account));
    }

    return () => unsub();
  }, [parachainApi, bonds, account]);
}
