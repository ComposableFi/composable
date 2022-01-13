import { expect } from "chai";


/**
 * 
**/
export class QueryTokenTests {
  public static runQueryTokenTests() {
    describe('query.token Tests', function() {
      // Asset tests
      it('Get single asset amount', async () => {
        await QueryTokenTests.getSingleAssetAmount(global.api, global.walletAlice.address);
      });
    
      it('Get list of asset amounts', async () => {
        await QueryTokenTests.getListAssetAmounts(global.api, global.walletAlice.address);
      });
    });
  }
  
  /**
   * Tests by checking the balance of the supplied account is >0
   * @param {ApiPromise} api Connected API Promise.
   * @param {string} walletAddress generated key through Keyring class
   */
  private static async getSingleAssetAmount(api, walletAddress:string) {
    const {free, reserved, frozen} = await api.query.tokens.accounts(walletAddress, 0);
    if (!free && !reserved && !frozen) {
      throw new Error('getSingleAsssetAmount: Received variable wasn\'t set!');
    }
    expect(parseInt(free)).to.be.a('number');
    expect(parseInt(reserved)).to.be.a('number');
    expect(parseInt(frozen)).to.be.a('number');
  }
  
  /**
   * Tests by checking the balance of the supplied account is >0
   * @param {ApiPromise} api Connected API Promise.
   * @param {string} walletAddress generated key through Keyring class
   */
  private static async getListAssetAmounts(api, walletAddress:string) {
    // ToDo (D. Roth): Change call to get all tokens, and add check.
    // const data = await api.query.tokens.accounts(WALLET_FERDIE_ADDR, 0);
  }
}