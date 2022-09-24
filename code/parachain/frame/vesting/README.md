# Vesting Pallet

*The vesting pallet adds functionality to gradually unlock an accounts balance*

---

## Overview

Vesting module provides a means of scheduled balance lock on an account. It uses the *graded vesting* way, which unlocks a specific amount of balance every period of time, until all balance unlocked.

### Vesting Schedule

The schedule of a vesting is described by data structure `VestingSchedule`: from the time of `window.start`, for every `window.period` amount of time, `per_period` amount of balance would unlocked, until number of periods `period_count` reached. The pallet supports measuring time windows in terms of absolute timestamps as well as block numbers for vesting schedules. All `VestingSchedule`s under an account could be queried in chain state.

### Why fork

This tweaked version includes the following changes,
1. The original Vesting pallet is not currency agnostic. This fork is generalized to any currency and usable with the `MultiLockableCurrency` trait. 
2. Modified to support measuring time in terms of absolute timestamps as well as the original block number based scheme for vesting schedules.

Other than that, most of the code is the original version from the [open runtime module repo](https://github.com/open-web3-stack/open-runtime-module-library/blob/1f520348f31b5e94b8a5dd7f8e6b8ec359df4177/vesting/README.md)
