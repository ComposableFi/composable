// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { CustomRpcBalance } from '@composable/types/interfaces/common';
import type { Bytes, Enum, Result, Struct, u32, u64 } from '@polkadot/types-codec';
import type { AccountId, Hash } from '@polkadot/types/interfaces/runtime';
import type { DispatchError } from '@polkadot/types/interfaces/system';

/** @name Code */
export interface Code extends Enum {
  readonly isUpload: boolean;
  readonly asUpload: Bytes;
  readonly isExisting: boolean;
  readonly asExisting: Hash;
  readonly type: 'Upload' | 'Existing';
}

/** @name ContractExecResult */
export interface ContractExecResult extends Struct {
  readonly gas_consumed: u64;
  readonly gas_required: u64;
  readonly storage_deposit: StorageDeposit;
  readonly debug_message: Bytes;
  readonly result: Result<ExecReturnValue, DispatchError>;
}

/** @name ContractInstantiateResult */
export interface ContractInstantiateResult extends Struct {
  readonly gas_consumed: u64;
  readonly gas_required: u64;
  readonly storage_deposit: StorageDeposit;
  readonly debug_message: Bytes;
  readonly result: Result<InstantiateReturnValue, DispatchError>;
}

/** @name ExecReturnValue */
export interface ExecReturnValue extends Struct {
  readonly flags: u32;
  readonly data: Bytes;
}

/** @name InstantiateReturnValue */
export interface InstantiateReturnValue extends Struct {
  readonly result: ExecReturnValue;
  readonly account_id: AccountId;
}

/** @name StorageDeposit */
export interface StorageDeposit extends Enum {
  readonly isRefund: boolean;
  readonly asRefund: CustomRpcBalance;
  readonly isCharge: boolean;
  readonly asCharge: CustomRpcBalance;
  readonly type: 'Refund' | 'Charge';
}

export type PHANTOM_COSMWASM = 'cosmwasm';
