import { IKeyringPair } from "@polkadot/types/types";
import Web3 from "web3";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { Bool, Option, u128, u64, Vec } from "@polkadot/types-codec";
import { sendAndWaitForSuccess, sendUnsignedAndWaitForSuccess } from "@composable/utils/polkadotjs";
import BN from "bn.js";
import { AnyString, ITuple } from "@polkadot/types-codec/types";
import { Moment } from "@polkadot/types/interfaces/runtime";
import { expect } from "chai";
import { PalletAirdropModelsIdentity, PalletAirdropModelsProof } from "@composable/types/interfaces";
import { AccountId32 } from "@polkadot/types/interfaces";

const toHexString = bytes => Array.prototype.map.call(bytes, x => ("0" + (x & 0xff).toString(16)).slice(-2)).join("");

export const proofMessage = (account: IKeyringPair, isEth = false) =>
  (isEth ? "picasso-" : "<Bytes>picasso-") + toHexString(account.publicKey) + (isEth ? "" : "</Bytes>");

export const ethAccount = (seed: number) =>
  new Web3().eth.accounts.privateKeyToAccount("0x" + seed.toString(16).padStart(64, "0"));

export class TxAirdropTests {

  /**
   * ToDo
   *
   * @param {ApiPromise} api Connected API Promise.
   * @param wallet
   * @param startAt
   * @param vestingSchedule
   */
  public static async createAirdrop(
    api: ApiPromise,
    wallet: KeyringPair,
    startAt: Option<u64>,
    vestingSchedule: u64
  ) {
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.airdrop.AirdropCreated.is,
      api.tx.airdrop.createAirdrop(startAt, vestingSchedule)
    );
  }

  /**
   * ToDo
   *
   * @param api
   * @param airdrop_id
   * @param airdropMaintainerPublicKey
   * @param startAt
   * @param vesting_period
   */
  public static async verifyAirdropCreation(
    api: ApiPromise,
    airdrop_id: u128 | BN,
    airdropMaintainerPublicKey: Uint8Array | AnyString,
    startAt: any, vesting_period: Moment | u64
  ) {
    /*
    ToDo: Update for different airdrops!
     */
    const airdropInformation = await api.query.airdrop.airdrops(airdrop_id);
    expect(airdropInformation.unwrap().creator).to.be.eql(api.createType("AccountId", airdropMaintainerPublicKey));
    expect(airdropInformation.unwrap().total_funds).to.be.eql(undefined);
    expect(airdropInformation.unwrap().total_recipients).to.be.eql(undefined);
    expect(airdropInformation.unwrap().start.isNone).to.be.true;
    expect(airdropInformation.unwrap().schedule).to.be.bignumber.equal(vesting_period);
    expect(airdropInformation.unwrap().disabled).to.be.eql(api.createType("bool", false));
  }

  /**
   * ToDo
   *
   * @param api
   * @param wallet
   * @param airdropId
   * @param recipients
   */
  public static async addRecipient(
    api: ApiPromise,
    wallet: KeyringPair,
    airdropId: u128 | BN,
    recipients: Vec<ITuple<[PalletAirdropModelsIdentity, u128, u64, Bool]>>
  ) {
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.airdrop.RecipientsAdded.is,
      api.tx.airdrop.addRecipient(airdropId, recipients)
    );
  }

  public static async removeRecipient(
    api: ApiPromise,
    wallet: KeyringPair,
    airdropId: u128 | BN,
    recipient: PalletAirdropModelsIdentity
  ) {
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.airdrop.RecipientRemoved.is,
      api.tx.airdrop.removeRecipient(airdropId, recipient)
    );
  }

  public static async enableAirdrop(
    api: ApiPromise,
    wallet: KeyringPair,
    airdropId: u128 | BN
  ) {
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.airdrop.AirdropStarted.is,
      api.tx.airdrop.enableAirdrop(airdropId)
    );
  }

  public static async disableAirdrop(
    api: ApiPromise,
    wallet: KeyringPair,
    airdropId: u128 | BN
  ) {
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.airdrop.AirdropEnded.is,
      api.tx.airdrop.disableAirdrop(airdropId)
    );
  }

  public static async claimAirdrop(
    api: ApiPromise,
    airdropId: u128 | BN,
    rewardAccount: AccountId32,
    proof: PalletAirdropModelsProof
  ) {
    return await sendUnsignedAndWaitForSuccess(
      api,
      api.events.airdrop.Claimed.is,
      api.tx.airdrop.claim(airdropId, rewardAccount, proof)
    );
  }
}
