/// <reference types="@composable/types/interfaces/types-lookup" />
/**
 * Tests tx.bondedFinance.cancel with provided parameters that should succeed.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64} offerId
 */
import { IKeyringPair } from "@polkadot/types/types";
export declare function txBondedFinanceCancelSuccessTest(wallet: IKeyringPair, offerId: any): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").u64]>>;
/**
 * Tests tx.bondedFinance.cancel with provided parameters that should fail.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64} offerId
 */
export declare function txBondedFinanceCancelFailureTest(wallet: IKeyringPair, offerId: any): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/lookup").SpRuntimeDispatchError, import("@polkadot/types/lookup").FrameSupportWeightsDispatchInfo]>>;
/**
 * Tests tx.bondedFinance.cancel as SUDO with provided parameters that should succeed.
 * @param {IKeyringPair} wallet Connected API Promise w/ sudo rights.
 * @param {u64} offerId
 */
export declare function txBondedFinanceCancelSudoSuccessTest(wallet: IKeyringPair, offerId: any): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").Result<import("@polkadot/types-codec").Null, import("@polkadot/types/lookup").SpRuntimeDispatchError>]>>;
