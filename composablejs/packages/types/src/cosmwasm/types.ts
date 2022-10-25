// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Enum, Null, Struct, u64 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';

/** @name PalletCosmwasmCodeInfo */
export interface PalletCosmwasmCodeInfo extends Null {}

/** @name PalletCosmwasmContractInfo */
export interface PalletCosmwasmContractInfo extends Struct {
  readonly codeId: u64;
}

/** @name PalletCosmwasmEntryPoint */
export interface PalletCosmwasmEntryPoint extends Enum {
  readonly isInstantiate: boolean;
  readonly asInstantiate: ITuple<[]>;
  readonly isExecute: boolean;
  readonly asExecute: ITuple<[]>;
  readonly isMigrate: boolean;
  readonly asMigrate: ITuple<[]>;
  readonly isReply: boolean;
  readonly asReply: ITuple<[]>;
  readonly isSudo: boolean;
  readonly asSudo: ITuple<[]>;
  readonly isQuery: boolean;
  readonly asQuery: ITuple<[]>;
  readonly type: 'Instantiate' | 'Execute' | 'Migrate' | 'Reply' | 'Sudo' | 'Query';
}

export type PHANTOM_COSMWASM = 'cosmwasm';
