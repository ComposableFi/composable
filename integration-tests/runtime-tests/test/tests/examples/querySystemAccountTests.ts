import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";

/**
 * Example Test
 * Just checking if provided wallet balance >0.
 */
// describe(name, function) groups all query tests for the system pallet.
describe("query.system Tests", function() {
  // Check if group of tests are enabled.
  if (!testConfiguration.enabledTests.query.enabled)
    return;

  // This describe groups all system.account query tests.
  describe("query.system.account Tests", function() {
    // Check if group of tests are enabled.
    if (!testConfiguration.enabledTests.query.account__success.enabled)
      return;

    // it(name, function) describes a single test.
    it("Wallet balance check should be >0", async function() {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1)
        this.skip();
      const balance = await QuerySystemAccountTests.checkBalance(api, walletAlice.address);
      expect(balance.free.toBigInt() > 0).to.be.true; // .to.be.greater(0) didn't work for some reason.
    });
  });
});

export class QuerySystemAccountTests {
  /**
   * Tests by checking the balance of the supplied account is >0
   * @param {ApiPromise} api Connected API Promise.
   * @param {string} walletAddress wallet public key
   */
  public static async checkBalance(api: ApiPromise, walletAddress: string) {
    const { data: balance } = await api.query.system.account(walletAddress);
    return balance;
  }
}
