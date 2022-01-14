/* eslint-disable no-trailing-spaces */
import {Promise} from 'bluebird';

Promise.config({
  // Enable warnings
  warnings: true,
  // Enable long stack traces
  longStackTraces: true,
  // Enable cancellation
  cancellation: true,
  // Enable monitoring
  monitoring: true,
  // Enable async hooks
  asyncHooks: true,
});

/**
 * Contains all TX tests for the pallet:
 * bondedFinance
 */
export class TxBondedFinanceTests {
  /**
   * Runs all tx tests for the bondedFinance pallet.
   */
  public static runTxBondedFinanceTests() {
    describe('tx.bondedFinance Tests', function () {
      this.timeout(0);
      it('tx.bondedFinance.offer', async function () {
        await TxBondedFinanceTests.txBondedFinanceOfferTest(global.walletAlice);
      });
    });
  }

  /**
   * Tests tx.bondedFinance.offer successfully. SUDO Check!
   */
  private static async txBondedFinanceOfferTest(wallet) {
    // ToDo: Find good parameter for reward.asset
    const requestParameters = {
      beneficiary: wallet.publicKey,
      asset: 1,
      bondPrice: 1,
      nbOfBonds: 10000000000000,
      maturity: {Finite:{returnIn:50}},
      reward: {asset: [170,141,183,460,469,231,731,687,303,715,884,105,728], amount: 10, maturity: 1}
    };
    try {
      return new Promise(function (resolve, reject) {
        global.api.tx.bondedFinance.offer(requestParameters)
        .signAndSend(wallet, { nonce: -1 }, ({ events=[], status }) => {
          console.debug('txBondedFinanceOfferTest: Transaction status: ', status.type);
          if (status.isFinalized) {
            events
            // find/filter for failed events
            .filter(({ event }) =>
              global.api.events.system.ExtrinsicFailed.is(event)
            )
            // we know that data for system.ExtrinsicFailed is
            // (DispatchError, DispatchInfo)
            .forEach(({ event: { data: [error, info] } }) => {
              if (error.isModule) {
                // for module errors, we have the section indexed, lookup
                const decoded = global.api.registry.findMetaError(error.asModule);
                const { docs, method, section } = decoded;

                console.log(`${section}.${method}: ${docs.join(' ')}`);
                throw new Error('txBondedFinanceOfferTest: ExtrinsicFailed!');
              } else {
                // Other, CannotLookup, BadOrigin, no extra info
                console.log(error.toString());
                throw new Error('txBondedFinanceOfferTest: ExtrinsicFailed!');
              }
            });
            // If no errors have occured, resolve promise.
            // ToDo (D. Roth): Add checks
            resolve();
          }
        });
      });
    } catch (exc) {
      console.error(exc);
    }
  }
}

// Uncomment to debug
TxBondedFinanceTests.runTxBondedFinanceTests();