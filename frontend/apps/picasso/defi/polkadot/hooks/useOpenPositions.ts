import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { useEffect } from "react";
import { usePicassoProvider } from "@/defi/polkadot/hooks/index";
import { ApiPromise } from "@polkadot/api";
import { unwrapNumberOrHex } from "shared";
import { useStore } from "@/stores/root";
import { Codec } from "@polkadot/types-codec/types";

type VestingAccount = { name: string; address: string };

const bondedVestingSchedule = (bond: BondOffer) => (address: string) => (
  api: ApiPromise
) => {
  return async () => {
    const vestingScheduleResponse: Codec = await api.query.vesting.vestingSchedules(
      address,
      bond.reward.assetId
    );

    if (vestingScheduleResponse.isEmpty) {
      return null;
    }
    const fromCodec: Array<any> = vestingScheduleResponse.toJSON() as any;

    return fromCodec.flatMap(vs => {
      if (vs) {
        const perPeriod = unwrapNumberOrHex((vs as any).perPeriod);
        const periodCount = unwrapNumberOrHex((vs as any).periodCount);
        const window = {
          blockNumberBased: {
            start: unwrapNumberOrHex((vs as any).window.blockNumberBased.start),
            period: unwrapNumberOrHex(
              (vs as any).window.blockNumberBased.period
            )
          }
        };
        return {
          bond,
          perPeriod,
          periodCount,
          window,
          type: window.blockNumberBased ? "block" : "time"
        };
      }
    });
  };
};

export function useOpenPositions(account: VestingAccount | undefined) {
  const bonds = useStore<BondOffer[]>(state => state.bonds.bonds);
  const updateOpenPositions = useStore(
    state => state.bonds.updateOpenPositions
  );
  const { parachainApi } = usePicassoProvider();

  // traverse to all bonds
  // get all reward assets
  // fetch each reward asset and create a new ar
  async function fetchVestingSchedules(api: ApiPromise, acc: VestingAccount) {
    const allVesting = bonds
      .flatMap(
        async bond => await bondedVestingSchedule(bond)(acc.address)(api)()
      )
      .flat();
    return Promise.all(allVesting);
  }

  async function fetchAndStore(factoryFn: () => Promise<unknown>) {
    const result: any = await factoryFn();
    updateOpenPositions(result.filter(Boolean).flat());
  }

  useEffect(() => {
    let unsub = () => {};
    if (parachainApi && account) {
      fetchAndStore(() => fetchVestingSchedules(parachainApi, account));
    }

    return () => unsub();
  }, [parachainApi, bonds, account]); // eslint-disable-line react-hooks/exhaustive-deps
}
