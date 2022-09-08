<!-- AUTOMATICALLY GENERATED -->
<!-- Generated at 2022-09-05T18:35:35.08578Z -->

# Vesting Pallet Extrinsics

## Claim

[`claim`](https://dali.devnets.composablefinance.ninja/doc/pallet_vesting/pallet/enum.Call.html#variant.claim)

Unlock any vested funds of the origin account.

The dispatch origin for this call must be *Signed* and the sender must have funds still
locked under this pallet.

* `asset`: The asset associated with the vesting schedule
* `vesting_schedule_ids`: The ids of the vesting schedules to be claimed

Emits `Claimed`.

## Vested Transfer

[`vested_transfer`](https://dali.devnets.composablefinance.ninja/doc/pallet_vesting/pallet/enum.Call.html#variant.vested_transfer)

Create a vested transfer.

The dispatch origin for this call must be *Signed*.

* `from`: The account sending the vested funds.
* `beneficiary`: The account receiving the vested funds.
* `asset`: The asset associated with this vesting schedule.
* `schedule_info`: The vesting schedule data attached to the transfer.

Emits `VestingScheduleAdded`.

NOTE: This will unlock all schedules through the current block.

## Update Vesting Schedules

[`update_vesting_schedules`](https://dali.devnets.composablefinance.ninja/doc/pallet_vesting/pallet/enum.Call.html#variant.update_vesting_schedules)

Update vesting schedules

The dispatch origin for this call must be *Signed*.

* `who`: The account whose vested funds should be updated.
* `asset`: The asset associated with the vesting schedules.
* `vesting_schedules`: The updated vesting schedules.

Emits `VestingSchedulesUpdated`.

## Claim For

[`claim_for`](https://dali.devnets.composablefinance.ninja/doc/pallet_vesting/pallet/enum.Call.html#variant.claim_for)

Unlock any vested funds of a `target` account.

The dispatch origin for this call must be *Signed*.

* `dest`: The account whose vested funds should be unlocked. Must have funds still
  locked under this pallet.
* `asset`: The asset associated with the vesting schedule.
* `vesting_schedule_ids`: The ids of the vesting schedules to be claimed.

Emits `Claimed`.
