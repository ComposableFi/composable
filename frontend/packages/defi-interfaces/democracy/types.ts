// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Enum, Null, Struct, Vec, u128 } from '@polkadot/types-codec';
import type { AccountId32, Balance } from '@polkadot/types/interfaces/runtime';

/** @name PalletDemocracyPreimageStatus */
export interface PalletDemocracyPreimageStatus extends Null {}

/** @name PalletDemocracyReferendumInfo */
export interface PalletDemocracyReferendumInfo extends Null {}

/** @name PalletDemocracyReleases */
export interface PalletDemocracyReleases extends Null {}

/** @name PalletDemocracyVoteThreshold */
export interface PalletDemocracyVoteThreshold extends Null {}

/** @name PalletDemocracyVoteVoting */
export interface PalletDemocracyVoteVoting extends Enum {
  readonly isDelegating: boolean;
  readonly asDelegating: {
    readonly balance: Balance;
    readonly target: AccountId32;
    readonly conviction: Null;
    readonly delegations: {
    readonly votes: Null;
    readonly capital: Null;
  } & Struct;
    readonly prior: Null;
  } & Struct;
  readonly isDirect: boolean;
  readonly asDirect: {
    readonly votes: Vec<Null>;
    readonly delegations: {
    readonly votes: u128;
    readonly capital: u128;
    readonly prior: Null;
  } & Struct;
  } & Struct;
  readonly type: 'Delegating' | 'Direct';
}

/** @name PalletPreimageRequestStatus */
export interface PalletPreimageRequestStatus extends Null {}

export type PHANTOM_DEMOCRACY = 'democracy';
