import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
/**
 * Tests tx.oracle.reclaimStake with provided parameters that should succeed.
 * @param api Connected ApiPromise
 * @param controller KeyringPair which is a controller.
 */
export declare function txOracleReclaimStakeSuccessTest(api: ApiPromise, controller: KeyringPair): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/interfaces").AccountId32, import("@polkadot/types-codec").u128]>>;
