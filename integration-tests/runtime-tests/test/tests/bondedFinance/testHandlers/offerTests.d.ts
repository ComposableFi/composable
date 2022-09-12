/// <reference types="@composable/types/interfaces/types-lookup" />
import { IKeyringPair } from "@polkadot/types/types";
import { ApiPromise } from "@polkadot/api";
import { u32, u128 } from "@polkadot/types-codec";
/**
 * Tests tx.bondedFinance.offer with provided parameters that should succeed.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param requestParameters wallet public key
 * @return Transaction event.
 */
export declare function txBondedFinanceOfferSuccessTest(api: ApiPromise, wallet: IKeyringPair, requestParameters: {
    beneficiary: Uint8Array;
    asset: u128;
    bondPrice: u128;
    nbOfBonds: u128;
    maturity: {
        Finite: {
            returnIn: u32;
        };
    };
    reward: {
        asset: u128;
        amount: u128;
        maturity: u32;
    };
}): Promise<import("@polkadot/types/types").IEvent<[u128, import("@polkadot/types/interfaces").AccountId32]>>;
/**
 * Tests tx.bondedFinance.offer with provided parameters that should fail.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param requestParameters wallet public key
 * @return Transaction event.
 */
export declare function txBondedFinanceOfferFailureTest(api: ApiPromise, wallet: IKeyringPair, requestParameters: any): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/lookup").SpRuntimeDispatchError, import("@polkadot/types/lookup").FrameSupportWeightsDispatchInfo]>>;
