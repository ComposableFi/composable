# Vesting Pallet

## Overview

Vesting module provides a means of scheduled balance lock on an account. It uses the *graded vesting* way, which unlocks a specific amount of balance every period of time, until all balance unlocked.

### Vesting Schedule

The schedule of a vesting is described by data structure `VestingSchedule`: from the block number of `start`, for every `period` amount of blocks, `per_period` amount of balance would unlocked, until number of periods `period_count` reached. Note in vesting schedules, *time* is measured by block number. All `VestingSchedule`s under an account could be queried in chain state.

### Why fork

The original Vesting pallet is not currency agnostic. This tweaked version is generalized to any currency and usable with the `MultiLockableCurrency` trait. Other than that, most of the code is the original version from the [open runtime module repo](https://github.com/open-web3-stack/open-runtime-module-library/blob/1f520348f31b5e94b8a5dd7f8e6b8ec359df4177/vesting/README.md)
