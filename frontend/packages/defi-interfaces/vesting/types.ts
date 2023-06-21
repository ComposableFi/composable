// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Enum, Null, Struct, u128, u32 } from '@polkadot/types-codec';
import type { Balance, BlockNumber, Moment } from '@polkadot/types/interfaces/runtime';

/** @name ComposableTraitsVestingVestingSchedule */
export interface ComposableTraitsVestingVestingSchedule extends Struct {
  readonly vestingScheduleId: u128;
  readonly window: VestingWindow;
  readonly periodCount: u32;
  readonly perPeriod: Balance;
  readonly alreadyClaimed: Balance;
}

/** @name ComposableTraitsVestingVestingScheduleIdSet */
export interface ComposableTraitsVestingVestingScheduleIdSet extends Null {}

/** @name ComposableTraitsVestingVestingScheduleInfo */
export interface ComposableTraitsVestingVestingScheduleInfo extends Struct {
  readonly window: VestingWindow;
  readonly periodCount: u128;
  readonly perPeriod: u128;
}

/** @name VestingWindow */
export interface VestingWindow extends Enum {
  readonly isMomentBased: boolean;
  readonly asMomentBased: {
    readonly start: Moment;
    readonly period: Moment;
  } & Struct;
  readonly isBlockNumberBased: boolean;
  readonly asBlockNumberBased: {
    readonly start: BlockNumber;
    readonly period: BlockNumber;
  } & Struct;
  readonly type: 'MomentBased' | 'BlockNumberBased';
}

export type PHANTOM_VESTING = 'vesting';
