/* eslint-disable no-trailing-spaces */
import {expect} from "chai";
import {
  txBondedFinanceCancelSudoSuccessTest,
  txBondedFinanceCancelFailureTest,
  txBondedFinanceCancelSuccessTest
} from '@composable/tests/tx/bondedFinance/testHandlers/cancelTests';
import {
  txBondedFinanceOfferFailureTest,
  txBondedFinanceOfferSuccessTest
} from "@composable/tests/tx/bondedFinance/testHandlers/offerTests";
import {txBondedFinanceBondSuccessTest} from "@composable/tests/tx/bondedFinance/testHandlers/bondTests";

/**
 * Contains all TX tests for the pallet:
 * bondedFinance
 */
export class TxBondedFinanceTests {
  /**
   * Runs all tx tests for the bondedFinance pallet.
   *
   * ToDo (D. Roth): The tests assume you're running them on a fresh chain. Instead of assuming, use the test returns.
   */
  public static runTxBondedFinanceTests() {
    describe('tx.bondedFinance Tests', function() {
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
          const { data: [result], } = await txBondedFinanceOfferSuccessTest(walletAlice, requestParameters);
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
          const { data: [result], } = await txBondedFinanceOfferSuccessTest(walletBob, requestParameters);
          expect(result.toNumber()).to.be.a('number');
        });

        /**
         * bondedFinance.bond(offerId:u64, nbOfBonds:u128) Tests
         */
        // #3 Bob can bond to the offer Alice has created.
        it('Can bond to newly created offer', async function () {
          const offerId = api.createType('u64', 1);
          const nbOfBonds = api.createType('u128', 1);
          await txBondedFinanceBondSuccessTest(walletBob, offerId, nbOfBonds);
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
          const {data: [result],} = await txBondedFinanceOfferFailureTest(walletAlice, requestParameters);
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
          const {data: [result],} = await txBondedFinanceOfferFailureTest(walletAlice, requestParameters);
          expect(result.toNumber()).to.be.a('number');
        });

        // #5 Alice can't create offer with the reward amount too low.
        it('Should not be able to create offer (reward.asset does not exist)', async function () {
          const requestParameters = {
            beneficiary: walletAlice.publicKey,
            asset: api.createType('u128', 1),
            bondPrice: api.consts.bondedFinance.stake,
            nbOfBonds: api.createType('u128', 10),
            maturity: {Finite: {returnIn: api.createType('u32', 16)}},
            reward: {
              asset: api.createType('u128', 1337),
              amount: api.consts.bondedFinance.minReward,
              maturity: api.createType('u32', 1)
            }
          };
          const {data: [result],} = await txBondedFinanceOfferFailureTest(walletAlice, requestParameters);
          expect(result.toNumber()).to.be.a('number');
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
          const { data: [result], } = await txBondedFinanceCancelFailureTest(walletAlice, offerId);
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
          const { data: [result], } = await txBondedFinanceCancelSuccessTest(walletAlice, offerId);
          expect(result.toNumber()).to.be.a('number');
        });

        // #7 A sudo command should be able to cancel an offer.
        it('Can sudo (diff. account) cancel offer created in second bondedFinance.offer', async function () {
          const offerId = 2;
          const { data: [result], } = await txBondedFinanceCancelSudoSuccessTest(walletAlice, offerId);
          expect(result.isOk).to.be.true;
        });
      });
    });
  }
}

// Uncomment to debug
//TxBondedFinanceTests.runTxBondedFinanceTests();
