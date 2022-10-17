import { KeyringPair } from "@polkadot/keyring/types";
import { sendAndWaitForSuccess, sendUnsignedAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { AnyNumber, IKeyringPair, ITuple } from "@polkadot/types/types";
import { PalletCrowdloanRewardsModelsRemoteAccount } from "@composable/types/interfaces";
import { Compact, u128, u32, u64, Vec } from "@polkadot/types-codec";
import { shares } from "@composabletests/tests/crowdloanRewards/contributions.json";
import { expect } from "chai";
import Web3 from "web3";
import { ApiPromise } from "@polkadot/api";
import BN from "bn.js";

const toHexString = (bytes: any) =>
  Array.prototype.map.call(bytes, x => ("0" + (x & 0xff).toString(16)).slice(-2)).join("");

// The prefix is defined as pallet config
const proofMessage = (account: IKeyringPair, isEth = false) =>
  (isEth ? "picasso-" : "<Bytes>picasso-") + toHexString(account.publicKey) + (isEth ? "" : "</Bytes>");

export const ethAccount = (seed: number) =>
  new Web3().eth.accounts.privateKeyToAccount("0x" + seed.toString(16).padStart(64, "0"));

export class TxCrowdloanRewardsTests {
  /**
   * Providing the crowdloan pallet with funds
   *
   * Unfortunately we can't directly mint into the pallet therefore,
   * we mint into the Alice wallet and transfer funds from there.
   *
   * @param {ApiPromise} api Connected API Client.
   * @param {KeyringPair} sudoKey Wallet with sudo rights.
   * @param amount
   */
  public static async beforeCrowdloanTestsProvideFunds(
    api: ApiPromise,
    sudoKey: KeyringPair,
    amount: u128 | Compact<u128> | AnyNumber
  ) {
    const palletPublicKey = api.consts.crowdloanRewards.accountId;
    return await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.balances.Transfer.is,
      api.tx.balances.transfer(palletPublicKey, amount)
    );
  }

  /**
   * tx.crowdloanRewards.initialize
   *
   * @param {ApiPromise} api Connected API Client.
   * @param {KeyringPair} sudoKey Wallet with sudo rights.
   */
  public static txCrowdloanRewardsInitializeTest(api: ApiPromise, sudoKey: KeyringPair) {
    return sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.crowdloanRewards.initialize())
    );
  }

  /**
   * tx.crowdloanRewards.populate
   *
   * @param {ApiPromise} api Connected API Client.
   * @param {KeyringPair} sudoKey Wallet with sudo rights.
   * @param testContributorWallet KSM Wallet of contributor to populate with.
   */
  public static async txCrowdloanRewardsPopulateTest(
    api: ApiPromise,
    sudoKey: KeyringPair,
    testContributorWallet: KeyringPair
  ) {
    const vesting48weeks = api.createType("u32", 100800);
    let contributors: Array<[PalletCrowdloanRewardsModelsRemoteAccount, u128, u32]> = [];
    // Before we go through all the contributors, we inject our test wallet at the very beginning.
    const testContributorReward = api.createType("u128", 1_000_000_000_000);
    const testContributorRelayChainObject = api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
      RelayChain: testContributorWallet.publicKey
    });
    const testContributorEthChainObject = api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
      Ethereum: ethAccount(1).address
    });
    contributors.push([testContributorRelayChainObject, testContributorReward, vesting48weeks]);
    contributors.push([testContributorEthChainObject, testContributorReward, vesting48weeks]);
    // Iterating through our list of contributors
    let i = 0;
    let amount: BN = new BN(testContributorReward.toNumber() * 2);
    for (const [key, value] of Object.entries(shares)) {
      let remoteAccountObject: PalletCrowdloanRewardsModelsRemoteAccount;
      // Creating either an ethereum or ksm contributor object.
      if (key.startsWith("0x"))
        remoteAccountObject = api.createType("PalletCrowdloanRewardsModelsRemoteAccount", { Ethereum: key });
      else
        remoteAccountObject = api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
          RelayChain: api.createType("AccountId32", key)
        });
      const currentContributorAmount = new BN((parseFloat(value) * Math.pow(10, 12)).toFixed(0));
      amount = currentContributorAmount.add(amount);
      contributors.push([remoteAccountObject, api.createType("u128", currentContributorAmount), vesting48weeks]);

      // Every 2500th iteration we send our list of contributors, else we'd break the block data size limit.
      if (i % 2500 == 0 && i != 0) {
        // Providing funds since calling `populate` verifies that the pallet funds are equal to contributor amount.
        const {
          data: [provideFundsResult]
        } = await TxCrowdloanRewardsTests.beforeCrowdloanTestsProvideFunds(
          api,
          sudoKey,
          api.createType("u128", amount)
        );
        expect(provideFundsResult).to.not.be.undefined;
        // Actual population step.
        const {
          data: [result]
        } = await TxCrowdloanRewardsTests.txCrowdloanRewardsPopulateTestHandler(api, sudoKey, contributors);
        expect(result.isOk).to.be.true;
        amount = new BN(0);
        contributors = [];
      }
      i++;
    }
    return testContributorRelayChainObject;
  }

  /**
   * tx.crowdloanRewards.populate
   *
   * @param {ApiClient} api Connected ApiClient
   * @param {KeyringPair} sudoKey Wallet with sudo rights.
   * @param {KeyringPair} contributors List of contributors to be transacted.
   */
  public static async txCrowdloanRewardsPopulateTestHandler(
    api: ApiPromise,
    sudoKey: KeyringPair,
    contributors:
      | [PalletCrowdloanRewardsModelsRemoteAccount, u128, u32][]
      | Vec<ITuple<[PalletCrowdloanRewardsModelsRemoteAccount, u128, u64]>>
      | [
          string | Uint8Array | PalletCrowdloanRewardsModelsRemoteAccount | { RelayChain: any } | { Ethereum: any },
          u128 | AnyNumber,
          AnyNumber | u64
        ][]
  ) {
    return await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.crowdloanRewards.populate(contributors))
    );
  }

  /**
   * tx.crowdloanRewards.associate RelayChain
   *
   * @param {ApiPromise} api Connected ApiPromise
   * @param {KeyringPair} contributor The contributor relay chain wallet public key.
   * @param {KeyringPair} contributorRewardAccount The wallet the contributor wants to receive their PICA to.
   */
  public static async txCrowdloanRewardsRelayAssociateTests(
    api: ApiPromise,
    contributor: KeyringPair,
    contributorRewardAccount: IKeyringPair
  ) {
    // arbitrary, user defined reward account
    const proof = contributor.sign(proofMessage(contributorRewardAccount));
    return await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(
        contributorRewardAccount.publicKey,
        api.createType("PalletCrowdloanRewardsModelsProof", { RelayChain: [contributor.publicKey, { Sr25519: proof }] })
      )
    );
  }

  /**
   * tx.crowdloanRewards.associate ETH Chain
   *
   * @param {ApiClient} api Connected ApiClient
   * @param {KeyringPair} contributor The contributor ETH chain wallet public key.
   * @param {KeyringPair} contributorRewardAccount The wallet the contributor wants to receive their PICA to.
   */
  public static async txCrowdloanRewardsEthAssociateTest(
    api: ApiPromise,
    contributor: { sign: (arg0: string) => any },
    contributorRewardAccount: IKeyringPair
  ) {
    const proof = contributor.sign(proofMessage(contributorRewardAccount, true));
    return await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(
        contributorRewardAccount.publicKey,
        api.createType("PalletCrowdloanRewardsModelsProof", { Ethereum: proof.signature })
      )
    );
  }

  /**
   * tx.crowdloanRewards.claim
   *
   * @param {ApiClient} api Connected ApiClient
   * @param { KeyringPair } wallet The reward account which tries to claim.
   */
  public static async txCrowdloanRewardsClaimTest(api: ApiPromise, wallet: KeyringPair) {
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.crowdloanRewards.Claimed.is,
      api.tx.crowdloanRewards.claim()
    );
  }
}
