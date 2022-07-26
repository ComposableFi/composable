Design Proposal: Pablo Fees & Staking Rewards Distribution
==========================================================

Table of Contents

- [1. Abstract](#1-abstract)
- [2. Background](#2-background)
- [3. Use Cases](#3-requirements)
- [4. Method](#4-method)
  - [4.1. Structs](#41-structs)
  - [4.2. Storage](#42-storage)
  - [4.3. Events](#43-events)
  - [4.4. Errors](#44-errors)
  - [4.5. Extrinsics](#45-extrinsics)
    - [4.5.1 vested_transfer](#451-vested_transfer)
    - [4.5.2 claim](#452-claim)

## 1. Abstract

This document proposes that vesting schedules are tracked individually, instead of being tracked as a group based on their asset id.

## 2. Background

Currently, the `vesting` pallet groups vesting schedules by `asset id`. This means that if a user has multiple
vesting schedules for the same asset, they will all be grouped together, and there will be no distinction between
them. Therefore, when a user wants to `claim`, it is only possible to claim **everything** that has been vested for
a given asset. We need to change this in order to allow users to claim only a chosen vesting schedule.

## 3. Requirements

The vesting pallet

1. MUST keep track of individual vesting schedules for each asset.
2. MUST allow a user to choose which vesting schedule to claim.
3. MUST include the claimed schedule in the corresponding event.
4. MUST not do anything when a non-existent vesting schedule is claimed.
5. SHOULD allow users to claim **all** vesting schedules for the same asset, by not specifying a `vesting_schedule_id`

## 4. Method

### 4.1. Structs

The `VestingSchedule` struct should include `pub vesting_schedule_id: VestingScheduleId,` where

```rust
type VestingScheduleId: Copy
    + Clone
    + Eq
    + Debug
    + Zero
    + SafeAdd
    + One
    + Ord
    + FullCodec
    + MaxEncodedLen
    + MaybeSerializeDeserialize
    + TypeInfo;
```

### 4.2. Storage

A counter should be used as a unique id for each `VestingSchedule` added.
```rust
#[pallet::storage]
#[pallet::getter(fn vesting_schedules_count)]
#[allow(clippy::disallowed_types)]
pub type VestingScheduleCount<T: Config> =
    StorageValue<_, T::VestingScheduleId, ValueQuery, Nonce<ZeroInit, SafeIncrement>>;
```

### 4.3. Events

- The `Claimed` event should include `vesting_schedule_id`, as this can be used on `Subsquid` to keep track of the
individual vesting schedules.

### 4.4. Errors

- A `VestingScheduleNotFound` error should be added to the pallet, and returned when a non-existent vesting schedule
is claimed.

- The `MaxVestingSchedulesExceeded` error should be thrown when `VestingScheduleCount` has reached its maximum value.

### 4.5. Extrinsics

The following changes should be made on the `vesting` pallet:

#### 4.5.1. `vested_transfer`
The `lock` MUST use the `VestingScheduleId` as key, instead of always using the same constant value.

#### 4.5.2. `claim`
It MUST accept `vesting_schedule_id` as an argument, and only remove the corresponding lock.


Last updated 2022-06-29 11:48:18 +0200
