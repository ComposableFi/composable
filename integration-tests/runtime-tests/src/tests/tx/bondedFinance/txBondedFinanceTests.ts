/* eslint-disable no-trailing-spaces */
import { sendAndWaitForSuccess } from '@composable/utils/polkadotjs';
import { IKeyringPair } from '@polkadot/types/types';

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
  private static txBondedFinanceOfferTest(wallet: IKeyringPair) {
    // ToDo: Find good parameter for reward.asset
    const requestParameters = {
      beneficiary: wallet.publicKey,
      asset: 1,
      bondPrice: api.consts.vesting.minVestedTransfer,
      nbOfBonds: 1,
      maturity: { Finite: { returnIn: 10 } },
      reward: { asset: 1, amount: 100000000000000, maturity: 1 } // pub MinReward: Balance = 10 * CurrencyId::PICA.unit::<Balance>();
    };
    return sendAndWaitForSuccess(
      global.api,
      wallet,
      global.api.events.bondedFinance.NewOffer.is,
      global.api.tx.bondedFinance.offer(requestParameters)
    );
  }
}

// Uncomment to debug
TxBondedFinanceTests.runTxBondedFinanceTests();
