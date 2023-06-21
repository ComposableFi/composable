// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Null, Struct, u128 } from '@polkadot/types-codec';

/** @name FrameSystemAccountInfo */
export interface FrameSystemAccountInfo extends Struct {
  readonly nonce: Null;
  readonly consumers: Null;
  readonly providers: Null;
  readonly sufficients: Null;
  readonly data: {
    readonly free: u128;
    readonly reserved: u128;
    readonly miscFrozen: u128;
    readonly feeFrozen: u128;
  } & Struct;
}

/** @name FrameSystemEventRecord */
export interface FrameSystemEventRecord extends Struct {
  readonly phase: Null;
  readonly event: {
    readonly section: Null;
    readonly method: Null;
  } & Struct;
  readonly topics: Null;
}

/** @name FrameSystemLastRuntimeUpgradeInfo */
export interface FrameSystemLastRuntimeUpgradeInfo extends Null {}

/** @name FrameSystemLimitsBlockLength */
export interface FrameSystemLimitsBlockLength extends Null {}

/** @name FrameSystemLimitsBlockWeights */
export interface FrameSystemLimitsBlockWeights extends Null {}

/** @name FrameSystemPhase */
export interface FrameSystemPhase extends Null {}

export type PHANTOM_SYSTEM = 'system';
