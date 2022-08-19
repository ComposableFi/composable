import { VestingSchedule } from "@/defi/types";
import { ApiPromise } from "@polkadot/api";
import { decodeVestingSchedule } from "./decode";

export async function fetchVestingSchedule(
  parachainApi: ApiPromise,
  address: string,
  currencyId: string
): Promise<VestingSchedule[]> {
  let schedules: VestingSchedule[] = [];

  try {
    const vestingSchedule = await parachainApi.query.vesting.vestingSchedules(
      address,
      currencyId
    );

    const _schedules = vestingSchedule.toJSON();
    schedules = Object.values(_schedules as any)
        .map((i) => decodeVestingSchedule(i))
  } catch (err: any) {
    console.error(err);
  }

  return schedules;
}
