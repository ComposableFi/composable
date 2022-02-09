/* eslint-disable no-trailing-spaces */
import R from 'ramda';
import { PalletCrowdloanRewardsModelsRemoteAccount } from '@composable/types/interfaces';
import { sendAndWaitForSuccess, sendUnsignedAndWaitForSuccess } from '@composable/utils/polkadotjs';
import { u128, u32 } from '@polkadot/types-codec';
import { expect } from 'chai';
import { IKeyringPair } from '@polkadot/types/types';
import {KeyringPair} from "@polkadot/keyring/types";

const toHexString = bytes =>
  Array.prototype.map.call(bytes, x => ('0' + (x & 0xFF).toString(16)).slice(-2)).join('');

// The prefix is defined as pallet config
const proofMessage = (account: IKeyringPair, isEth=false) =>
  (isEth ? "picasso-" : "<Bytes>picasso-") + toHexString(account.publicKey) + (isEth ? "" : "</Bytes>");

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
        const sudoWallet = walletAlice;
        const { data: [result], } = await TxCrowdloanRewardsTests.txCrowdloanRewardsPopulateTest(sudoWallet);
        expect(result.isOk).to.be.true;
      });

      it('Can initialize the crowdloan', async function() {
        const sudoWallet = walletAlice;
        const { data: [result], } = await TxCrowdloanRewardsTests.txCrowdloanRewardsInitializeTest(sudoWallet);
        expect(result.isOk).to.be.true;
      });

      it('Can associate a picasso account', async function() {
        const wallet = walletAlice;
        await Promise.all([
          TxCrowdloanRewardsTests.txCrowdloanRewardsEthAssociateTests(wallet),
          TxCrowdloanRewardsTests.txCrowdloanRewardsRelayAssociateTests(wallet),
        ]);
      });
    });
  }

  /**
   * tx.crowdloanRewards.populate
   */
  private static txCrowdloanRewardsInitializeTest(sudoKey:KeyringPair) {
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
  private static async txCrowdloanRewardsPopulateTest(sudoKey:KeyringPair) {
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

  private static async txCrowdloanRewardsRelayAssociateTests(wallet:KeyringPair) {
    const contributor = wallet.derive("/contributor-1");
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

  private static async txCrowdloanRewardsEthAssociateTests(wallet:KeyringPair) {
    const contributor = ethAccount(1);
    // arbitrary, user defined reward account
    const contributorRewardAccount = wallet.derive("/reward-eth-1");
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
