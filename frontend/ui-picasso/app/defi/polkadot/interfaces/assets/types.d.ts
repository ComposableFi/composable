import type { Bytes, Struct, u64 } from "@polkadot/types-codec";
/** @name Asset */
export interface Asset extends Struct {
    readonly name: Bytes;
    readonly id: u64;
}
export declare type PHANTOM_ASSETS = "assets";
