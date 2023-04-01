# Vesting

The vesting module provides a means for a scheduled balance lock on an account.
It utilizes the `graded vesting` approach, which unlocks a specific amount of balance every period of time until all 
funds are unlocked.

## Overview

This pallet is a fork of the [open runtime module repo](https://github.com/open-web3-stack/open-runtime-module-library/blob/1f520348f31b5e94b8a5dd7f8e6b8ec359df4177/vesting/README.md) and contains the following changes:
- The original pallet is not currency agnostic. This fork is generalized to any currency and usable with the 
  `MultiLockableCurrency` trait.
- The pallet is modified to support measuring time windows in absolute timestamps and block numbers.

## Vesting Schedule

The data structure `VestingSchedule` describes the schedule of a vesting plan:
1. from the time of `window.start`, start of first period (cannot claim until first period ends)
2. for every `window.period` amount of time,
3. `per_period` amount of balance is unlocked, until
4. the number of periods 'period_count' is reached.  

All `VestingSchedules` under an account can be queried from the chain state.

## Workflows

Initially, we create a `vested_transfer` to add a vesting schedule to an account. 
Once created, a vesting schedule can be updated with `update_vesting_schedules`.

A third party pallet would implement `VestedTransfer` as a dependency to execute vested transfers.

Funds can be claimed in two ways; either directly with `claim` to claim for the caller, or indirectly through 
`claim_for` to claim for a given account.