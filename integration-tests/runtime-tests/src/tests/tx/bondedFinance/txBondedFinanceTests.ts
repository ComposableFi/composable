/* eslint-disable no-trailing-spaces */
import { sendAndWaitForSuccess } from '@composable/utils/polkadotjs';
import {IKeyringPair} from '@polkadot/types/types';
import {expect} from "chai";

/**
 * Contains all TX tests for the pallet:
 * bondedFinance
 *
 * ToDo (D. Roth): Split this up into multiple files for each function. To keep it clean.
 */
export class TxBondedFinanceTests {
  /**
   * Runs all tx tests for the bondedFinance pallet.
   *
   * ToDo (D. Roth): The tests assume you're running them on a fresh chain. Instead of assuming, use the test returns.
   */
  public static runTxBondedFinanceTests() {
    /**
     * bondedFinance.offer(...) Success Tests
     */
    describe('tx.bondedFinance.offer & .bond Success Tests', function () {
      // Timeout set to 2 minutes
      this.timeout(2*60*1000);
      // #1 Create offer using Alice's wallet.
      it('Can create a new offer', async function () {
        const requestParameters = {
          beneficiary: walletAlice.publicKey,
          asset: api.createType('u128', 1),
          bondPrice: api.consts.bondedFinance.stake,
          nbOfBonds: api.createType('u128', 10),
          maturity: { Finite: { returnIn: api.createType('u32', 16) } },
          reward: {
            asset: api.createType('u128', 1),
            amount: api.consts.bondedFinance.minReward,
            maturity: api.createType('u32', 1)
          }
        };
        const { data: [result], } = await TxBondedFinanceTests.txBondedFinanceOfferSuccessTest(walletAlice, requestParameters);
        expect(result.toNumber()).to.be.a('number');
      });

      // #2 Create offer using Bob's wallet.
      it('Can create a second new offer', async function () {
        const requestParameters = {
          beneficiary: walletBob.publicKey,
          asset: api.createType('u128', 1),
          bondPrice: api.consts.bondedFinance.stake,
          nbOfBonds: api.createType('u128', 10),
          maturity: { Finite: { returnIn: api.createType('u32', 16) } },
          reward: {
            asset: api.createType('u128', 1),
            amount: api.consts.bondedFinance.minReward,
            maturity: api.createType('u32', 1)
          }
        };
        const { data: [result], } = await TxBondedFinanceTests.txBondedFinanceOfferSuccessTest(walletBob, requestParameters);
        expect(result.toNumber()).to.be.a('number');
      });

      /**
       * bondedFinance.bond(offerId:u64, nbOfBonds:u128) Tests
       */
      // #3 Bob can bond to the offer Alice has created.
      it('Can bond to newly created offer', async function () {
        const offerId = api.createType('u64', 1);
        const nbOfBonds = api.createType('u128', 1);
        await TxBondedFinanceTests.txBondedFinanceBondSuccessTest(walletBob, offerId, nbOfBonds);
      });
    });

    /**
     * Runs all tx FAILURE tests for the bondedFinance pallet.
     */
    describe('tx.bondedFinance.offer Failure Tests', function () {
      // Timeout set to 2 minutes
      this.timeout(2*60*1000);
      // #4 Alice can't create am offer with the bond price too low.
      it('Should not be able to create offer (bondPrice < MIN_VESTED_TRANSFER)', async function () {
        const requestParameters = {
          beneficiary: walletAlice.publicKey,
          asset: api.createType('u128', 1),
          bondPrice: api.createType('u128', api.consts.bondedFinance.stake.toNumber()-1),
          nbOfBonds: api.createType('u128', 10),
          maturity: {Finite: {returnIn: api.createType('u32', 16)}},
          reward: {
            asset: api.createType('u128', 1),
            amount: api.consts.bondedFinance.minReward,
            maturity: api.createType('u32', 1)
          }
        };
        const {data: [result],} = await TxBondedFinanceTests.txBondedFinanceOfferFailureTest(walletAlice, requestParameters);
        expect(result.toNumber()).to.be.a('number');
      });

      // #5 Alice can't create offer with the reward amount too low.
      it('Should not be able to create offer (reward.amount < MinReward)', async function () {
        const requestParameters = {
          beneficiary: walletAlice.publicKey,
          asset: api.createType('u128', 1),
          bondPrice: api.consts.bondedFinance.stake,
          nbOfBonds: api.createType('u128', 10),
          maturity: {Finite: {returnIn: api.createType('u32', 16)}},
          reward: {
            asset: api.createType('u128', 1),
            amount: api.createType('u128', api.consts.bondedFinance.minReward.toNumber()-1),
            maturity: api.createType('u32', 1)
          }
        };
        const {data: [result],} = await TxBondedFinanceTests.txBondedFinanceOfferFailureTest(walletAlice, requestParameters);
        expect(result.toNumber()).to.be.a('number');
      });
    });

    /**
     * Runs SUCCESS tests for bondedFinance.cancel(offerId)
     */
    describe('tx.bondedFinance.cancel Success Tests', function () {
      // Timeout set to 2 minutes
      this.timeout(2*60*1000);
      // #6 Alice should be able to cancel her offer.
      it('Can cancel offer created in first bondedFinance.offer test by creator', async function () {
        const offerId = 1;
        const { data: [result], } = await TxBondedFinanceTests.txBondedFinanceCancelSuccessTest(walletAlice, offerId);
        console.debug(result);
        expect(result.toNumber()).to.be.a('number');
      });

      // #7 A sudo command should be able to cancel an offer.
      it('Can sudo (diff. account) cancel offer created in second bondedFinance.offer', async function () {
        const offerId = 2;
        const { data: [result], } = await TxBondedFinanceTests.txBondedFinanceCancelSudoSuccessTest(walletAlice, offerId);
        expect(result.isOk).to.be.true;
      });
    });

    /**
     * Runs FAILURE tests for bondedFinance.cancel(offerId)
     */
    describe('tx.bondedFinance.cancel Failure Tests', function () {
      // Timeout set to 2 minutes
      this.timeout(2*60*1000);
      it('Should not be able to cancel offer that doesn\'t exist', async function () {
        const offerId = 1337;
        const { data: [result], } = await TxBondedFinanceTests.txBondedFinanceCancelFailureTest(walletAlice, offerId);
        expect(result.toNumber()).to.be.a('number');
      });
    });
  }

  /**
   * The implementation of the tests start here.
   */

  /**
   * Tests tx.bondedFinance.offer with provided parameters that should succeed.
   * @param {IKeyringPair} wallet Connected API Promise.
   * @param {} requestParameters wallet public key
   */
  private static txBondedFinanceOfferSuccessTest(wallet: IKeyringPair, requestParameters) {
    return sendAndWaitForSuccess(
      api,
      wallet,
      api.events.bondedFinance.NewOffer.is,
      api.tx.bondedFinance.offer(requestParameters),
    );
  }

  /**
   * Tests tx.bondedFinance.offer with provided parameters that should fail.
   * @param {IKeyringPair} wallet Connected API Promise.
   * @param {} requestParameters wallet public key
   */
  private static txBondedFinanceOfferFailureTest(wallet: IKeyringPair, requestParameters) {
    return sendAndWaitForSuccess(
      api,
      wallet,
      api.events.system.ExtrinsicFailed.is,
      api.tx.bondedFinance.offer(requestParameters),
      true
    );
  }

  /**
   * Tests tx.bondedFinance.offer with provided parameters that should succeed.
   * @param {IKeyringPair} wallet Connected API Promise.
   * @param {u64} offerId
   * @param {u128} nbOfBonds
   */
  private static txBondedFinanceBondSuccessTest(wallet: IKeyringPair, offerId, nbOfBonds) {
    return sendAndWaitForSuccess(
      api,
      wallet,
      api.events.bondedFinance.NewBond.is,
      api.tx.bondedFinance.bond(offerId, nbOfBonds),
    );
  }

  /**
   * Tests tx.bondedFinance.offer with provided parameters that should fail.
   * @param {IKeyringPair} wallet Connected API Promise.
   * @param {u64} offerId
   * @param {u128} nbOfBonds
   */
  private static txBondedFinanceBondFailureTest(wallet: IKeyringPair, offerId, nbOfBonds) {
    return sendAndWaitForSuccess(
      api,
      wallet,
      api.events.system.ExtrinsicFailed.is,
      api.tx.bondedFinance.bond(offerId, nbOfBonds),
      true
    );
  }

  /**
   * Tests tx.bondedFinance.cancel with provided parameters that should succeed.
   * @param {IKeyringPair} wallet Connected API Promise.
   * @param {u64} offerId
   */
  private static txBondedFinanceCancelSuccessTest(wallet: IKeyringPair, offerId) {
    return sendAndWaitForSuccess(
      api,
      wallet,
      api.events.bondedFinance.OfferCancelled.is,
      api.tx.bondedFinance.cancel(offerId),
    );
  }

  /**
   * Tests tx.bondedFinance.cancel with provided parameters that should fail.
   * @param {IKeyringPair} wallet Connected API Promise.
   * @param {u64} offerId
   */
  private static txBondedFinanceCancelFailureTest(wallet: IKeyringPair, offerId) {
    return sendAndWaitForSuccess(
      api,
      wallet,
      api.events.system.ExtrinsicFailed.is,
      api.tx.bondedFinance.cancel(offerId),
      true
    );
  }

  /**
   * Tests tx.bondedFinance.cancel as SUDO with provided parameters that should succeed.
   * @param {IKeyringPair} wallet Connected API Promise w/ sudo rights.
   * @param {u64} offerId
   */
  private static txBondedFinanceCancelSudoSuccessTest(wallet: IKeyringPair, offerId) {
    return sendAndWaitForSuccess(
      api,
      wallet,
      api.events.sudo.Sudid.is, api.tx.sudo.sudo(
        api.tx.bondedFinance.cancel(offerId),
      )
    );
  }
}

// Uncomment to debug
//TxBondedFinanceTests.runTxBondedFinanceTests();
