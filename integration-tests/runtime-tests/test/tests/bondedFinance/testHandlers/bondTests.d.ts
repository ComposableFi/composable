/// <reference types="@composable/types/interfaces/types-lookup" />
import { IKeyringPair } from "@polkadot/types/types";
/**
 * Tests tx.bondedFinance.offer with provided parameters that should succeed.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64} offerId
 * @param {u128} nbOfBonds
 */
export declare function txBondedFinanceBondSuccessTest(wallet: IKeyringPair, offerId: any, nbOfBonds: any): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").u64, import("@polkadot/types/interfaces").AccountId32, import("@polkadot/types-codec").u128]>>;
/**
 * Tests tx.bondedFinance.offer with provided parameters that should fail.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64} offerId
 * @param {u128} nbOfBonds
 */
export declare function txBondedFinanceBondFailureTest(wallet: IKeyringPair, offerId: any, nbOfBonds: any): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/lookup").SpRuntimeDispatchError, import("@polkadot/types/lookup").FrameSupportWeightsDispatchInfo]>>;
