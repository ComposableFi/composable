import R from 'ramda';
import { PalletCrowdloanRewardsModelsRemoteAccount } from '@composable/types/interfaces';
import { u128, u32 } from '@polkadot/types-codec';
import { expect } from 'chai';
import { IKeyringPair } from '@polkadot/types/types';
import {KeyringPair} from "@polkadot/keyring/types";
import testConfiguration from './test_configuration.json';
import {sendAndWaitForSuccess, sendUnsignedAndWaitForSuccess} from "@composable/utils/polkadotjs";
import {mintAssetsToWallet} from "@composable/utils/mintingHelper";

const toHexString = bytes =>
  Array.prototype.map.call(bytes, x => ('0' + (x & 0xFF).toString(16)).slice(-2)).join('');

// The prefix is defined as pallet config
const proofMessage = (account: IKeyringPair, isEth=false) =>
  (isEth ? "picasso-" : "<Bytes>picasso-") + toHexString(account.publicKey) + (isEth ? "" : "</Bytes>");

const ethAccount = (seed: number) =>
  web3.eth.accounts.privateKeyToAccount("0x" + seed.toString(16).padStart(64, '0'));


/**
 * Task order list:
 *  1. Provide funds to crowdloan pallet
 *  2. Populate the list of contributors
 *  3. Initialize the crowdloan
 *  4. Associate a picassso account (which also claims)
 *  5. Claiming more rewards.
 */
describe('CrowdloanRewards Tests', function() {
  if (!testConfiguration.enabledTests.tx.enabled)
    return;

  let wallet: KeyringPair,
    sudoKey: KeyringPair,
    contributor: KeyringPair,
    contributorRewardAccount: KeyringPair,
    contributorEth,
    contributorEthRewardAccount: KeyringPair;

  let onExistingChain = false;

  /**
   * We mainly set up some variables here.
   *
   * We also identify if this chain had already tests run on it.
   * And if so, we skip the populate() and initialize() tests.
   */
  before('Initializing Variables', async function() {
    sudoKey = walletAlice;
    wallet = walletAlice;
    let associationExisting = true;
    let i = 1;
    while (associationExisting) {
      contributor = wallet.derive("/contributor-" + i);
      contributorEth = ethAccount(i);
      // arbitrary, user defined reward account
      contributorRewardAccount = contributor.derive("/reward");
      contributorEthRewardAccount = wallet.derive("/reward-eth-" + i);
      const existingAssociations = await api.query.crowdloanRewards.associations(contributorRewardAccount.publicKey);
      if (existingAssociations.toString() == "") {
        associationExisting = false;
      } else {
        onExistingChain = true;
      }
      i++;
    }
    if (onExistingChain)
      console.info("tx.crowdloanRewards Tests: Detected already configured chain! " +
        "Skipping populate() & initialize().")
  });

  before('Providing funds to crowdloan pallet', async function() {
    if (!testConfiguration.enabledTests.tx.setup.provideAssets)
      this.skip();
    // 2 minutes timeout
    this.timeout(60 * 2 * 1000);
    const {data: [resultAccountId1, resultAccountId2, resultNumber],}
      = await TxCrowdloanRewardsTests.beforeCrowdlownTestsProvideFunds(sudoKey);
    expect(resultAccountId1.toString()).to.be
      .equal(api.createType('AccountId32', walletAlice.publicKey).toString());
    expect(resultNumber).to.be.a.bignumber;
  });

  /**
   * Here we populate the crowdloan pallet with a random generated list of contributors.
   *
   * This is a SUDO call and is checked with `.isOk`.
   */
  it('Can populate the list of contributors', async function() {
    if (!testConfiguration.enabledTests.tx.populate_success.populate1 || onExistingChain)
      this.skip();
    // 2 minutes timeout
    this.timeout(60 * 2 * 1000);
    const { data: [result,], } = await TxCrowdloanRewardsTests.txCrowdloanRewardsPopulateTest(sudoKey, walletAlice);
    expect(result.isOk).to.be.true;
  });

  /**
   * Here we initialize the crowdloan with our populated list of contributors.
   *
   * This is a SUDO call and is checked with `.isOk`.
   */
  it('Can initialize the crowdloan', async function() {
    if (!testConfiguration.enabledTests.tx.initialize_success.initialize1 || onExistingChain)
      this.skip();
    // 2 minutes timeout
    this.timeout(60 * 2 * 1000);
    const { data: [result], } = await TxCrowdloanRewardsTests.txCrowdloanRewardsInitializeTest(sudoKey);
    expect(result.isOk).to.be.true;
  });

  /***
   * Here we associate our picasso account with our ETH & RelayChain wallets.
   *
   * Here we send 2 transactions at the same time, therefore we have 2 results,
   * though with the exact same result structure.
   *
   * Results:
   * 1. The public key of the remote wallet.
   * 2. The public key of the transacting wallet.
   */
  it('Can associate a picasso account', async function() {
    if (!testConfiguration.enabledTests.tx.associate_success.associate1)
      this.skip();
    // 2 minutes timeout
    this.timeout(60 * 20 * 1000);
    await Promise.all([
      TxCrowdloanRewardsTests.txCrowdloanRewardsEthAssociateTest(
        contributorEth,
        contributorEthRewardAccount
      ),
      TxCrowdloanRewardsTests.txCrowdloanRewardsRelayAssociateTests(
        contributor,
        contributorRewardAccount
      ),
    ]).then(function([result, result2]) {
      expect(result.data[1].toString()).to.be
        .equal(api.createType('AccountId32', contributorEthRewardAccount.publicKey).toString());
      expect(result2.data[1].toString()).to.be
        .equal(api.createType('AccountId32', contributorRewardAccount.publicKey).toString());
    });
  });

  /**
   * Can we finally claim the crowdloan reward?
   * We're gonna find out!
   *
   * Results are:
   * 1. The public key of the remote account.
   * 2. The public key of the transacting wallet.
   * 3. The claimed amount.
   */
  it('Can claim the crowdloan reward', async function() {
    if (!testConfiguration.enabledTests.tx.claim_success.claim1 || onExistingChain)
      this.skip();
    // 2 minutes timeout
    this.timeout(60 * 2 * 1000);
    const { data: [resultRemoteAccountId, resultAccountId, resultClaimedAmount], }
      = await TxCrowdloanRewardsTests.txCrowdloanRewardsClaimTest(contributorRewardAccount);
    expect(resultClaimedAmount).to.be.a.bignumber;
    expect(resultAccountId.toString()).to.be
      .equal(api.createType('AccountId32', contributorRewardAccount.publicKey).toString())
  });
});

export class TxCrowdloanRewardsTests {
  /**
   * Providing the crowdloan pallet with funds
   *
   * Unfortunately we can't directly mint into the pallet therefore,
   * we mint into the Alice wallet and transfer funds from there.
   *
   * @param {KeyringPair} sudoKey Wallet with sudo rights.
   */
  public static async beforeCrowdlownTestsProvideFunds(sudoKey:KeyringPair) {
    const palletPublicKey = api.consts.crowdloanRewards.accountId;
    await mintAssetsToWallet(sudoKey, sudoKey, [1]);
    return await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.balances.Transfer.is,
      api.tx.balances.transfer(palletPublicKey, 100000000000000)
    );
  }

  /**
   * tx.crowdloanRewards.initialize
   *
   * @param {KeyringPair} sudoKey Wallet with sudo rights.
   */
  public static txCrowdloanRewardsInitializeTest(sudoKey:KeyringPair) {
    return sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(
        api.tx.crowdloanRewards.initialize()
      )
    );
  }

  /**
   * tx.crowdloanRewards.populate
   *
   * @param {KeyringPair} sudoKey Wallet with sudo rights.
   * @param {KeyringPair} contributor The picasso wallet of which the contributor wallets are derived from.
   */
  public static async txCrowdloanRewardsPopulateTest(sudoKey:KeyringPair, contributor:KeyringPair) {
    const vesting48weeks = api.createType('u32', 100800);
    const reward = api.createType('u128', 1_000_000_000_000);
    const relay_accounts =
      R.unfold<number, [PalletCrowdloanRewardsModelsRemoteAccount, u128, u32]>(n => n > 50 ? false : [[
        api.createType(
          'PalletCrowdloanRewardsModelsRemoteAccount',
          { RelayChain: contributor.derive("/contributor-" + n.toString()).publicKey }
        ),
        reward,
        vesting48weeks,
      ], n + 1], 1);
    const eth_accounts =
      R.unfold<number, [PalletCrowdloanRewardsModelsRemoteAccount, u128, u32]>(n => n > 50 ? false : [[
        api.createType(
          'PalletCrowdloanRewardsModelsRemoteAccount',
          { Ethereum: ethAccount(n).address }
        ),
        reward,
        vesting48weeks,
      ], n + 1], 1);
    const accounts = relay_accounts.concat(eth_accounts);
    return await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(
        api.tx.crowdloanRewards.populate(accounts)
      )
    );
  }

  /**
   * tx.crowdloanRewards.associate RelayChain
   *
   * @param {KeyringPair} contributor The contributor relay chain wallet public key.
   * @param {KeyringPair} contributorRewardAccount The wallet the contributor wants to receive their PICA to.
   */
  public static async txCrowdloanRewardsRelayAssociateTests(contributor:KeyringPair, contributorRewardAccount) {
    // arbitrary, user defined reward account
    const proof = contributor.sign(proofMessage(contributorRewardAccount));
    return await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(
        contributorRewardAccount.publicKey,
        api.createType('PalletCrowdloanRewardsModelsProof',
          { RelayChain: [contributor.publicKey, { Sr25519: proof }] })
      )
    );
  }

  /**
   * tx.crowdloanRewards.associate ETH Chain
   *
   * @param {KeyringPair} contributor The contributor ETH chain wallet public key.
   * @param {KeyringPair} contributorRewardAccount The wallet the contributor wants to receive their PICA to.
   */
  public static async txCrowdloanRewardsEthAssociateTest(contributor, contributorRewardAccount) {
    const proof = contributor.sign(proofMessage(contributorRewardAccount, true));
    return await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(
        contributorRewardAccount.publicKey,
        api.createType('PalletCrowdloanRewardsModelsProof', { Ethereum: proof.signature })
      )
    );
  }

  /**
   * tx.crowdloanRewards.claim
   *
   * @param { KeyringPair } wallet The reward account which tries to claim.
   */
  public static async txCrowdloanRewardsClaimTest(wallet:KeyringPair) {
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.crowdloanRewards.Claimed.is,
      api.tx.crowdloanRewards.claim()
    );
  }
}
