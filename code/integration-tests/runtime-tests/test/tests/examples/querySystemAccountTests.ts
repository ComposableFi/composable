import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";

/**
 * Example Test
 * Just checks if provided wallet balance >0.
 *
 * Feel free to use this as a starting template for your tests.
 *
 * For a more advanced example of a full test suite check out: `tests/oracle/txOracleTests.ts`.
 */
// describe(name, function) groups all query tests for the system pallet.
describe("query.system Tests", function () {
  // Check if group of tests are enabled.
  if (!testConfiguration.enabledTests.query.enabled) return;

  let api: ApiPromise;
  let walletAlice: KeyringPair;

  before("Setting up the tests", async function () {
    this.timeout(60 * 1000);
    // `getNewConnection()` establishes a new connection to the chain and gives us the ApiPromise & a Keyring.
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    // Using `getDevWallets(Keyring)` we're able to get a dict of all developer wallets.
    const { devWalletAlice } = getDevWallets(newKeyring);
    walletAlice = devWalletAlice;
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  // Another describe groups all system.account query tests.
  describe("query.system.account Tests", function () {
    // Check if group of tests are enabled.
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    // it(name, function) describes a single test.
    it("Wallet balance check should be >0", async function () {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();

      // Here we make our request
      const balance = await QuerySystemAccountTests.checkBalance(api, walletAlice.address);

      /*
      Finally, making our comparisons to verify everything was successful.

      Most of the time we can't rely on just the transaction result.
      Therefore, we add additional verification steps like balance checks, to make sure everything has worked fine.
       */
      expect(balance.free.toBigInt() > 0).to.be.true; // .to.be.greater(0) didn't work for some reason.
    });
  });
});

/**
 * If the test file is quite small like this one, we often have the request functions within the same file.
 * Though for big files, like `txOracleTests.ts`, we outsource the tests handlers into an extra subdirectory
 * called `testHandlers`.
 */
class QuerySystemAccountTests {
  /**
   * Sends a requests for `query.system.account` using the provided `walletAddress`
   *
   * @param {ApiPromise} api Connected API Promise.
   * @param {Uint8Array|string} walletAddress wallet public key
   */
  public static async checkBalance(api: ApiPromise, walletAddress: Uint8Array | string) {
    const { data: balance } = await api.query.system.account(walletAddress);
    return balance;
  }
}
