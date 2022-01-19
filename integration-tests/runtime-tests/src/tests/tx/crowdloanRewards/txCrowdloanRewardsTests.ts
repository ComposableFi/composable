/* eslint-disable no-trailing-spaces */
import R from 'ramda';
import { PalletCrowdloanRewardsModelsRemoteAccount } from '@composable/types/interfaces';
import { sendAndWaitForSuccess, sendUnsignedAndWaitForSuccess } from '@composable/utils/polkadotjs';
import { u128, u32 } from '@polkadot/types-codec';
import { expect } from 'chai';
import { IKeyringPair } from '@polkadot/types/types';

const toHexString = bytes =>
  Array.prototype.map.call(bytes, x => ('0' + (x & 0xFF).toString(16)).slice(-2)).join('');

// The prefix is defined as pallet config
const proofMessage = (account: IKeyringPair) =>
  "picasso-" + toHexString(account.publicKey);

const ethAccount = (seed: number) =>
  web3.eth.accounts.privateKeyToAccount("0x" + seed.toString(16).padStart(64, '0'))

export class TxCrowdloanRewardsTests {
  /**
   * Task order list:
   *  * Populate the list of contributors
   *  * Initialize the crowdloan
   *  * Associate a picassso account
   */
  public static runTxCrowdloanRewardsTests() {
    describe('CrowdloanRewards Tests', function() {
      // 2 minutes timeout
      this.timeout(60 * 2 * 1000);
      it('Can populate the list of contributors', async function() {
        const { data: [result], } = await TxCrowdloanRewardsTests.txCrowdloanRewardsPopulateTest();
        expect(result.isOk).to.be.true;
      });
      it('Can initialize the crowdloan', async function() {
        const { data: [result], } = await TxCrowdloanRewardsTests.txCrowdloanRewardsInitializeTest();
        expect(result.isOk).to.be.true;
      });
      it('Can associate a picasso account', async function() {
        await Promise.all([
          TxCrowdloanRewardsTests.txCrowdloanRewardsEthAssociateTests(),
          TxCrowdloanRewardsTests.txCrowdloanRewardsRelayAssociateTests(),
        ]);
      });
    });
  }

  /**
   * tx.crowdloanRewards.populate
   */
  private static txCrowdloanRewardsInitializeTest() {
    // ToDo (D. Roth): Pass api and keyring instead of directly reading from global.
    const sudoKey = walletAlice;
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
   *
   */
  private static async txCrowdloanRewardsPopulateTest() {
    // ToDo (D. Roth): Pass api and keyring instead of directly reading from global.
    const sudoKey = walletAlice;
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

  private static async txCrowdloanRewardsRelayAssociateTests() {
    const contributor = walletAlice.derive("/contributor-1");
    // arbitrary, user defined reward account
    const contributorRewardAccount = contributor.derive("/reward");
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

  private static async txCrowdloanRewardsEthAssociateTests() {
    const contributor = ethAccount(1);
    // arbitrary, user defined reward account
    const contributorRewardAccount = walletAlice.derive("/reward-eth-1");
    const proof = contributor.sign(proofMessage(contributorRewardAccount));
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
