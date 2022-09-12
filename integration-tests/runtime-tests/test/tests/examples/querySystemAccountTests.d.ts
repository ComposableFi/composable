/// <reference types="@composable/types/interfaces/types-lookup" />
import { ApiPromise } from "@polkadot/api";
/**
 * If the test file is quite small like this one, we often have the request functions within the same file.
 * Though for big files, like `txOracleTests.ts`, we outsource the tests handlers into an extra subdirectory
 * called `testHandlers`.
 */
export declare class QuerySystemAccountTests {
    /**
     * Sends a requests for `query.system.account` using the provided `walletAddress`
     *
     * @param {ApiPromise} api Connected API Promise.
     * @param {Uint8Array|string} walletAddress wallet public key
     */
    static checkBalance(api: ApiPromise, walletAddress: Uint8Array | string): Promise<import("@polkadot/types/lookup").PalletBalancesAccountData>;
}
