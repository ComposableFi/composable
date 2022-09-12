import { IKeyringPair } from "@polkadot/types/types";
import { ApiPromise } from "@polkadot/api";
import { u128, u64 } from "@polkadot/types-codec";
/**
 * Tests tx.bondedFinance.offer with provided parameters that should succeed.
 *
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64} offerId
 * @param {u128|number} nbOfBonds
 * @return Transaction event.
 */
export declare function txBondedFinanceBondSuccessTest(api: ApiPromise, wallet: IKeyringPair, offerId: u64, nbOfBonds: u128 | number): Promise<import("@polkadot/types/types").IEvent<[u128, import("@polkadot/types/interfaces").AccountId32, u128]>>;
