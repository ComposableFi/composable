// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Null, Option, Struct, bool, u32 } from '@polkadot/types-codec';
import type { AccountId, Balance, Moment } from '@polkadot/types/interfaces/runtime';
import type { Period } from '@polkadot/types/interfaces/scheduler';

/** @name PalletAirdropModelsAirdrop */
export interface PalletAirdropModelsAirdrop extends Struct {
  readonly creator: AccountId;
  readonly total_funds: Balance;
  readonly total_recipients: u32;
  readonly claimed_funds: Balance;
  readonly start: Option<Moment>;
  readonly schedule: Moment;
  readonly disabled: bool;
}

/** @name PalletAirdropModelsIdentity */
export interface PalletAirdropModelsIdentity extends Null {}

/** @name PalletAirdropModelsProof */
export interface PalletAirdropModelsProof extends Null {}

/** @name PalletAirdropModelsRecipientFund */
export interface PalletAirdropModelsRecipientFund extends Struct {
  readonly total: Balance;
  readonly claimed: Balance;
  readonly vesting_period: Period;
  readonly funded_claim: bool;
}

export type PHANTOM_AIRDROP = 'airdrop';
