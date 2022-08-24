# Vesting

## Overview
The vesting module provides a means to put a scheduled balance lock on an account. 
It uses the graded vesting approach, which unlocks a specific amount of balance every period of time until all funds are unlocked.

This pallet is a fork of the [open runtime module repo](https://github.com/open-web3-stack/open-runtime-module-library/blob/1f520348f31b5e94b8a5dd7f8e6b8ec359df4177/vesting/README.md) and contains the following changes:
- The original pallet is not currency agnostic. This fork is generalized to any currency and usable with the `MultiLockableCurrency` trait.
- The pallet is modified to support measuring time windows in absolute timestamps and block numbers.

## Vesting Schedule
The data structure `VestingSchedule` describes the schedule of a vesting plan:
1. from the time of `window.start`,
2. for every `window.period` amount of time,
3. `per_period` an amount of balance would unlock, until
4. the number of periods 'period_count' is reached.  

Querying all `VestingSchedules` under an account could be done in chain state.