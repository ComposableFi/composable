/// <reference types="@composable/types/interfaces/types-lookup" />
export declare function runBeforeTxOracleSetSigner(sudoKey: any, signer: any): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").Result<import("@polkadot/types-codec").Null, import("@polkadot/types/lookup").SpRuntimeDispatchError>]>>;
/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param controller Keyring which is a controller.
 * @param signer Keyring which will be set as a signer.
 */
export declare function txOracleSetSignerSuccessTest(controller: any, signer: any): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/interfaces").AccountId32, import("@polkadot/types/interfaces").AccountId32]>>;
