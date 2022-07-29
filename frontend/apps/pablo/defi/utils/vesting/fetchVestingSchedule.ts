import { ApiPromise } from "@polkadot/api";

export async function fetchVestingSchedule(
  parachainApi: ApiPromise,
  address: string,
  principalCurrencyId: string
) {
  const [vestingSchedule] = await parachainApi.query.vesting.vestingSchedules(
    address,
    principalCurrencyId
  ) as any;

  console.log("Vesting schedule", vestingSchedule);

  return vestingSchedule ? vestingSchedule : null;
}
