/// <reference types="@composable/types/interfaces/types-lookup" />
import { IKeyringPair } from "@polkadot/types/types";
/**
 * Tests tx.bondedFinance.offer with provided parameters that should succeed.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {} requestParameters wallet public key
 */
export declare function txBondedFinanceOfferSuccessTest(wallet: IKeyringPair, requestParameters: any): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").u64]>>;
/**
 * Tests tx.bondedFinance.offer with provided parameters that should fail.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {} requestParameters wallet public key
 */
export declare function txBondedFinanceOfferFailureTest(wallet: IKeyringPair, requestParameters: any): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/lookup").SpRuntimeDispatchError, import("@polkadot/types/lookup").FrameSupportWeightsDispatchInfo]>>;
