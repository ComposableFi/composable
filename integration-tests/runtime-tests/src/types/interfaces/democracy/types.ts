// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Null, Struct, Vec, u128 } from '@polkadot/types-codec';

/** @name PalletDemocracyPreimageStatus */
export interface PalletDemocracyPreimageStatus extends Null {}

/** @name PalletDemocracyReferendumInfo */
export interface PalletDemocracyReferendumInfo extends Null {}

/** @name PalletDemocracyReleases */
export interface PalletDemocracyReleases extends Null {}

/** @name PalletDemocracyVoteThreshold */
export interface PalletDemocracyVoteThreshold extends Null {}

/** @name PalletDemocracyVoteVoting */
export interface PalletDemocracyVoteVoting extends Struct {
  readonly direct: {
    readonly votes: Vec<Null>;
    readonly delegations: {
    readonly votes: u128;
    readonly capital: u128;
    readonly prior: Null;
  } & Struct;
  } & Struct;
}

/** @name PalletPreimageRequestStatus */
export interface PalletPreimageRequestStatus extends Null {}

export type PHANTOM_DEMOCRACY = 'democracy';
