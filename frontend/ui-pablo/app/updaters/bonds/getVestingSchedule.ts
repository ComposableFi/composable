import { ApiPromise } from "@polkadot/api";

export async function getVestingSchedule(
  parachainApi: ApiPromise,
  address: string,
  principalCurrencyId: string
) {
  const [vestingSchedule] = await parachainApi.query.vesting.vestingSchedules(
    address,
    principalCurrencyId
  );

  console.log("Vesting schedule", vestingSchedule);

  return vestingSchedule ? vestingSchedule : null;
}
