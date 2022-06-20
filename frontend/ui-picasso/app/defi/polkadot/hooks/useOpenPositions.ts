import { useAppDispatch, useAppSelector } from "@/hooks/store";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { useEffect } from "react";
import { usePicassoProvider } from "@/defi/polkadot/hooks/index";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { updateOpenPositions, VestingSchedule } from "@/stores/defi/polkadot/bonds/slice";

type VestingAccount = { name: string; address: string };

const bondedVestingSchedule =
  (bond: BondOffer) => (address: string) => (api: ApiPromise) =>
    api.query.vesting.vestingSchedules(address, bond.reward.assetId);

function unwrapNumberOrHex(v: string | number) {
  return new BigNumber(v, v.toString().startsWith("0x") ? 16 : 10);
}

export function useOpenPositions(account: VestingAccount | undefined) {
  const bonds = useAppSelector<BondOffer[]>((state) => state.bonding.bonds);
  const { parachainApi } = usePicassoProvider();
  const dispatch = useAppDispatch();

  // traverse to all bonds
  // get all reward assets
  // fetch each reward asset and create a new ar
  function fetchVestingSchedules(
    api: ApiPromise,
    acc: VestingAccount
  ): Promise<
    Awaited<{
      periodCount: BigNumber;
      perPeriod: BigNumber;
      window: {
        blockNumberBased: { period: BigNumber; start: BigNumber };
      };
      type: string;
      bond: BondOffer;
    } | null>[]
  > {
    const output = bonds
      .map(async (bond) => {
        const [vestingScheduleResponse] = await bondedVestingSchedule(bond)(
          acc.address
        )(api);
        const jsonVestingSchedule: VestingSchedule | null =
          vestingScheduleResponse.toJSON();
        if (jsonVestingSchedule) {
          const perPeriod = unwrapNumberOrHex(
            (jsonVestingSchedule as any).perPeriod
          );
          const periodCount = unwrapNumberOrHex(
            (jsonVestingSchedule as any).periodCount
          );
          console.log({
            window: (jsonVestingSchedule as any).window
          });
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
      .filter(Boolean);
    return Promise.all(output);
  }

  async function fetchAndStore(
    factoryFn: () => Promise<
      Awaited<{
        periodCount: BigNumber;
        perPeriod: BigNumber;
        window: {
          blockNumberBased: { period: BigNumber; start: BigNumber };
        };
        bond: BondOffer;
      } | null>[]
    >
  ) {
    const result = await factoryFn();
    dispatch(updateOpenPositions(result));
  }

  useEffect(() => {
    let unsub = () => {};
    if (parachainApi && account) {
      fetchAndStore(() => fetchVestingSchedules(parachainApi, account));
    }

    return () => unsub();
  }, [parachainApi, bonds, account]);
}
