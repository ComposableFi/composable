import R from 'ramda';
import { PalletCrowdloanRewardsModelsRemoteAccount } from '@composable/types/interfaces';
import { u128, u32 } from '@polkadot/types-codec';
import { expect } from 'chai';
import { IKeyringPair } from '@polkadot/types/types';
import {KeyringPair} from "@polkadot/keyring/types";
import testConfiguration from './test_configuration.json';
import {sendAndWaitForSuccess, sendUnsignedAndWaitForSuccess} from "@composable/utils/polkadotjs";

const toHexString = bytes =>
  Array.prototype.map.call(bytes, x => ('0' + (x & 0xFF).toString(16)).slice(-2)).join('');

// The prefix is defined as pallet config
const proofMessage = (account: IKeyringPair, isEth=false) =>
  (isEth ? "picasso-" : "<Bytes>picasso-") + toHexString(account.publicKey) + (isEth ? "" : "</Bytes>");

const ethAccount = (seed: number) =>
  web3.eth.accounts.privateKeyToAccount("0x" + seed.toString(16).padStart(64, '0'));

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
   * We identify if this chain had already tests run on it.
   * And if so, we skip the populate() and initialize() tests.
   */
  before(async function() {
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
  // 2 minutes timeout
  this.timeout(60 * 2 * 1000);
  it('Can populate the list of contributors', async function() {
    if (!testConfiguration.enabledTests.tx.populate_success.populate1 || onExistingChain)
      this.skip();
    const { data: [result], } = await TxCrowdloanRewardsTests.txCrowdloanRewardsPopulateTest(sudoKey);
    expect(result.isOk).to.be.true;
  });

  it('Can initialize the crowdloan', async function() {
    if (!testConfiguration.enabledTests.tx.initialize_success.initialize1 || onExistingChain)
      this.skip();
    const { data: [result], } = await TxCrowdloanRewardsTests.txCrowdloanRewardsInitializeTest(sudoKey);
    expect(result.isOk).to.be.true;
  });

  it('Can associate a picasso account', async function() {
    if (!testConfiguration.enabledTests.tx.associate_success.associate1)
      this.skip();
    await Promise.all([
      TxCrowdloanRewardsTests.txCrowdloanRewardsEthAssociateTests(
        wallet,
        contributorEth,
        contributorEthRewardAccount
      ),
      TxCrowdloanRewardsTests.txCrowdloanRewardsRelayAssociateTests(
        wallet,
        contributor,
        contributorRewardAccount
      ),
    ]).then(function(result) {
      expect(result[0].data[1].toString()).to.be
        .equal(api.createType('AccountId32', contributorEthRewardAccount.publicKey).toString());
      expect(result[1].data[1].toString()).to.be
        .equal(api.createType('AccountId32', contributorRewardAccount.publicKey).toString());
    });
  });
});

export class TxCrowdloanRewardsTests {
  /**
   * Task order list:
   *  * Populate the list of contributors
   *  * Initialize the crowdloan
   *  * Associate a picassso account
   */

  /**
   * tx.crowdloanRewards.initialize
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
   */
  public static async txCrowdloanRewardsPopulateTest(sudoKey:KeyringPair) {
    const vesting48weeks = api.createType('u32', 100800);
    const reward = api.createType('u128', 1_000_000_000_000);
    const relay_accounts =
      R.unfold<number, [PalletCrowdloanRewardsModelsRemoteAccount, u128, u32]>(n => n > 50 ? false : [[
        api.createType(
          'PalletCrowdloanRewardsModelsRemoteAccount',
          { RelayChain: walletAlice.derive("/contributor-" + n.toString()).publicKey }
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
      api.events.sudo.Sudid.is, api.tx.sudo.sudo(
        api.tx.crowdloanRewards.populate(accounts)
      )
    );
  }

  /**
   * tx.crowdloanRewards.associate RelayChain
   * @param { KeyringPair } wallet
   * @param contributor
   * @param contributorRewardAccount
   */
  public static async txCrowdloanRewardsRelayAssociateTests(wallet:KeyringPair, contributor, contributorRewardAccount) {
    // arbitrary, user defined reward account
    const proof = contributor.sign(proofMessage(contributorRewardAccount));
    return await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(
        contributorRewardAccount.publicKey,
        { RelayChain: [contributor.publicKey, { Sr25519: proof }] }
      )
    );
  }

  /**
   * tx.crowdloanRewards.associate ETH Chain
   * @param { KeyringPair } wallet
   * @param contributor
   * @param contributorRewardAccount
   */
  public static async txCrowdloanRewardsEthAssociateTests(wallet:KeyringPair, contributor, contributorRewardAccount) {
    const proof = contributor.sign(proofMessage(contributorRewardAccount, true));
    return await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(
        contributorRewardAccount.publicKey,
        { Ethereum: proof.signature }
      )
    );
  }
}
