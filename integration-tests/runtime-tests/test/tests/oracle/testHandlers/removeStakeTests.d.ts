import { ApiPromise } from "@polkadot/api";
/**
 * Tests tx.oracle.removeStake with provided parameters that should succeed.
 * @param controller KeyringPair which is a controller.
 */
export declare function txOracleRemoveStakeSuccessTest(api: ApiPromise, controller: any): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/interfaces").AccountId32, import("@polkadot/types-codec").u128, import("@polkadot/types-codec").u32]>>;
