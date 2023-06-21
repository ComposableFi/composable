/**
 * Helper functions to handle the oracle setup.
 */
import { ApiPromise, Keyring } from "@polkadot/api";
import { getNewConnection, sendAndWaitForSuccess } from "@composable/utils";
import { Bytes, Text, u128, u32 } from "@polkadot/types-codec";
import { AnyNumber, AnyString } from "@polkadot/types-codec/types";
import { AccountId32, Percent } from "@polkadot/types/interfaces";
import { AddressOrPair } from "@polkadot/api/types";
import { stringToHex } from "@polkadot/util";
import { expect } from "chai";
import { KeyringPair } from "@polkadot/keyring/types";

/**
 * Establishes connection.
 *
 * @returns {
 * api: ApiPromise,
 * keyring: Keyring,
 * walletAlice: KeyringPair,
 * }
 */
export async function connect(): Promise<{ api: ApiPromise; keyring: Keyring }> {
  const endpoint = "ws://" + (process.env.ENDPOINT ?? "127.0.0.1:9988");
  const { newClient, newKeyring } = await getNewConnection(endpoint);
  const api = newClient;
  const keyring = newKeyring;
  return { api: api, keyring: keyring };
}

/**
 * Creates Oracle for the given asset ID.
 *
 * @param {ApiPromise} api Connected API client.
 * @param {AddressOrPair} wallet KeyringPair object of the wallet to use.
 * @param oracleParameters Dict of all parameters to pass to new oracle.
 */
export async function createOracleForAsset(
  api: ApiPromise,
  wallet: AddressOrPair,
  oracleParameters: {
    assetID: number;
    threshold: Percent | AnyNumber;
    minAnswers: u32 | AnyNumber;
    maxAnswers: u32 | AnyNumber;
    blockInterval: u32 | AnyNumber;
    reward: u128 | AnyNumber;
    slash: u128 | AnyNumber;
  }
) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(
      api.tx.oracle.addAssetAndInfo(
        oracleParameters.assetID,
        oracleParameters.threshold,
        oracleParameters.minAnswers,
        oracleParameters.maxAnswers,
        oracleParameters.blockInterval,
        oracleParameters.reward,
        oracleParameters.slash,
        true
      )
    )
  );
}

/**
 * Sets url for the locally running oracle price feed.
 *
 * @param {ApiPromise} api Connected API client.
 * @param {number | Uint8Array | StorageKind | "PERSISTENT" | "LOCAL"} kind Type of storage, we only here "PERSISTENT" here.
 * @param {string|Uint8Array|Bytes} key The key to store the value under.
 * @param {string | Uint8Array | Bytes} value URL to price feed server.
 */
export async function setOracleURL(
  api: ApiPromise,
  key: string | Uint8Array | Bytes,
  value: string | Uint8Array | Bytes
) {
  return await api.rpc.offchain.localStorageSet(
    "PERSISTENT",
    stringToHex(<AnyString>key),
    stringToHex(<AnyString>value)
  );
}

/**
 * Registers off chain worker for the given oracle.
 *
 * @param {ApiPromise} api Connected API client.
 * @param {string} suri Private key of oracle.
 * @param {string | (Uint8Array | Bytes)} publicKey Public key of oracle.
 */
export async function registerOffChainWorker(
  api: ApiPromise,
  suri: string | Text,
  publicKey: string | Uint8Array | Bytes
) {
  return await api.rpc.author.insertKey(api.createType("Text", "orac"), api.createType("Text", suri), publicKey);
}

export async function verifyOffChainWorkerRegister(api: ApiPromise, publicKey: string | Uint8Array | Bytes) {
  const hasKey = await api.rpc.author.hasKey(publicKey, "orac");
  expect(hasKey.isTrue).to.be.true;
}

/**
 * Setting oracle signers.
 *
 * @param {ApiPromise} api Connected API client.
 * @param {AddressOrPair} controllerWallet KeyringPair of the controller wallet / previous signer.
 * @param {string | Uint8Array | AccountId32} signerWallet Public key of signer to set to.
 */
export async function setOracleSigner(
  api: ApiPromise,
  controllerWallet: AddressOrPair,
  signerWallet: string | Uint8Array | AccountId32
) {
  return await sendAndWaitForSuccess(
    api,
    controllerWallet,
    api.events.oracle.SignerSet.is,
    api.tx.oracle.setSigner(signerWallet)
  );
}

/**
 * Verifies return of the setSigner function.
 *
 * @param {ApiPromise} api Connected API client.
 * @param {AccountId32} resultAccount0 AccountId32 of new signer.
 * @param {AccountId32} resultAccount1 AccountId32 of previous signer/controller.
 * @param {AnyString} signerWalletKey Public key of new signer.
 * @param {KeyringPair} walletAlice Wallet of controller.
 */
export function verifyOracleSigner(
  api: ApiPromise,
  resultAccount0: AccountId32,
  resultAccount1: AccountId32,
  signerWalletKey: AnyString,
  walletAlice: KeyringPair
) {
  expect(resultAccount0).to.not.be.an("Error");
  expect(resultAccount1).to.not.be.an("Error");
  expect(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", signerWalletKey).toString());
  expect(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", walletAlice.publicKey).toString());
}

/**
 * Adds stake to the oracle for given wallet.
 *
 * @param {ApiPromise} api Connected API client.
 * @param {AddressOrPair} signer KeyringPair of the signer to add stake.
 * @param {u128 | AnyNumber} amount Amount of stake to add.
 */
export async function addOracleStake(api: ApiPromise, signer: AddressOrPair, amount: u128 | AnyNumber) {
  return await sendAndWaitForSuccess(api, signer, api.events.oracle.StakeAdded.is, api.tx.oracle.addStake(amount));
}

/**
 * Verifies addOracleStake ran successfully.
 *
 * @param api Connected API client.
 * @param result Result of add stake request.
 * @param signerWalletPublicKey public key wallet that has added stake.
 */
export function verifyAddOracleStake(
  api: ApiPromise,
  result: AccountId32,
  signerWalletPublicKey: AccountId32 | Uint8Array
) {
  expect(result).to.not.be.an("Error");
  expect(result.toString()).to.be.equal(api.createType("AccountId32", signerWalletPublicKey).toString());
}
