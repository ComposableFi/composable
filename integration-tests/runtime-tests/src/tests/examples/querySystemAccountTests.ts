/* eslint-disable no-trailing-spaces */
import { expect } from 'chai';


export class QuerySystemAccountTests {
  /**
   * Example Test
   * Just checking if provided wallet balance >0.
   */
  static runQuerySystemAccountTests() {
    describe('query.system.account Tests', function () {
      it('Wallet balance check should result >0', async () => {
        await QuerySystemAccountTests.checkBalance(global.api, global.walletAlice.address);
      });
    });
  }

  /**
  * Tests by checking the balance of the supplied account is >0
  * @param {ApiPromise} api Connected API Promise.
  * @param {string} walletAddress wallet public key
  */
  private static async checkBalance(api, walletAddress:string) {
    const {nonce, data: balance} = await api.query.system.account(walletAddress);
    expect(parseInt(balance.free)).to.be.a('number');
    expect(parseInt(nonce)).to.be.a('number');
    expect(parseInt(balance.free)).to.be.greaterThan(0);
  }
}