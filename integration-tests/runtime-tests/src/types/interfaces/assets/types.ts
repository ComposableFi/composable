// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

<<<<<<< HEAD
<<<<<<< HEAD
import type { u128, u64, Struct, Vec, u8 } from '@polkadot/types-codec';
=======
import type { u128, u64, Struct } from '@polkadot/types-codec';
>>>>>>> d9b5d7e5 (resolve conflicts)
=======
import type { u128, u64, Struct, Vec, u8 } from '@polkadot/types-codec';
>>>>>>> 14a330e0 (resolved coflicts)

/** @name AssetsBalance */
export interface AssetsBalance extends u128 {}

/** @name CurrencyId */
export interface CurrencyId extends u128 {}

export type PHANTOM_ASSETS = 'assets';

/** @name Asset */
export interface Asset extends Struct {
<<<<<<< HEAD
<<<<<<< HEAD
    readonly name: Vec<u8>;
=======
    readonly name: Text;
>>>>>>> d9b5d7e5 (resolve conflicts)
=======
    readonly name: Vec<u8>;
>>>>>>> 14a330e0 (resolved coflicts)
    readonly id: u64;
}