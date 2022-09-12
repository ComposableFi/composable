import { u128 } from "@polkadot/types-codec";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
/**
 * Provides funds for Oracle tests.
 * @param api Connect ApiPromise
 * @param sudoKey KeyringPair with sudo rights
 * @param wallet1 Wallet to provide funds to
 * @param wallet2 Wallet to provide funds to
 */
export declare function runBeforeTxOracleAddStake(api: ApiPromise, sudoKey: KeyringPair, wallet1: KeyringPair, wallet2: KeyringPair): Promise<void>;
/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param api Connect ApiPromise
 * @param sender Connected API Promise w/ sudo rights.
 * @param {u128} stake Staking amount.
 */
export declare function txOracleAddStakeSuccessTest(api: ApiPromise, sender: KeyringPair, stake: u128): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/interfaces").AccountId32, u128, u128]>>;
