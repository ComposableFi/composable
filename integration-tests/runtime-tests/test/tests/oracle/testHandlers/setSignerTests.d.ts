import { KeyringPair } from "@polkadot/keyring/types";
import { ApiPromise } from "@polkadot/api";
/**
 * Provides funds for Oracle tests.
 * @param api Connect ApiPromise
 * @param sudoKey KeyringPair with sudo rights
 * @param wallet1 Wallet to provide funds to
 * @param wallet2 Wallet to provide funds to
 */
export declare function runBeforeTxOracleSetSigner(api: ApiPromise, sudoKey: KeyringPair, signer: KeyringPair): Promise<void>;
/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param controller Keyring which is a controller.
 * @param signer Keyring which will be set as a signer.
 */
export declare function txOracleSetSignerSuccessTest(api: ApiPromise, controller: KeyringPair, signer: KeyringPair): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/interfaces").AccountId32, import("@polkadot/types/interfaces").AccountId32]>>;
