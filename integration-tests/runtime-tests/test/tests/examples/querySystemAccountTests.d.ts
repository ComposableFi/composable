import { ApiPromise } from "@polkadot/api";
export declare class QuerySystemAccountTests {
    /**
    * Tests by checking the balance of the supplied account is >0
    * @param {ApiPromise} api Connected API Promise.
    * @param {string} walletAddress wallet public key
    */
    static checkBalance(api: ApiPromise, walletAddress: string): Promise<void>;
}
