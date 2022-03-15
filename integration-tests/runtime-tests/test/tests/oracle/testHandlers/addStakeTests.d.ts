import { u128 } from "@polkadot/types-codec";
export declare function runBeforeTxOracleAddStake(sudoKey: any, wallet1: any, wallet2: any): Promise<void>;
/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param sender Connected API Promise w/ sudo rights.
 * @param {u128} stake Staking amount.
 */
export declare function txOracleAddStakeSuccessTest(sender: any, stake: u128): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/interfaces").AccountId32, u128, u128]>>;
