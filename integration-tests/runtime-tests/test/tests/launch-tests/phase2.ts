import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { createConsProdPool } from "@composabletests/tests/pablo/testHandlers/pabloTestHelper";

/**
 * Test suite for verifying phase 2 of the launch process.
 *
 * 2A. Seed KSM/USDC pool
 *  - Pool config: 50/50 Uniswap AMM w/ 0.15% fee.
 *  - Tests add/remove liquidity to/from the pool by users.
 *  - Tests stake/unstake LP tokens by users.
 *  - Tests pool receiving farming rewards.
 *  - Tests trading fees & distribution.
 *  - No users are allowed to create own pools during this phase.
 * 2B. Launch PICA via LBP event
 *  - Pool consists of USDC only.
 *  - Pool starts 98/2, finishing at 50/50.
 * 2C. Seed PICA/USDC pool
 *  - Pool config: 50/50 Uniswap AMM w/ 0.2% fee.
 *  - KSDM/USDC remains unchanged.
 *  - Pool receives additional PBLO farming rewards.
 *  - PICA/KSM will be created.
 * 2D. Add multiple pools
 *  - USDC/aUSD
 *  - - Stableswap AMM, 0.1% fee.
 *  - wETH/KSM
 *  - - Uniswap 50/50 AMM, 0.15% fee.
 *  - wBTC/KSM
 *  - - Uniswap 50/50 AMM, 0.15% fee.
 *  - USDC/USDT
 *  - - Stableswap AMM, 0.1% fee.
 */
describe("Picasso/Pablo Launch Plan - Phase 2", function() {
  if (!testConfiguration.enabledTests.query.enabled) return;

  let api: ApiPromise;
  let composableManagerWallet: KeyringPair;
  let ksmUsdcPoolId: number;

  before("Setting up the tests", async function() {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    const { devWalletAlice } = getDevWallets(newKeyring);
    composableManagerWallet = devWalletAlice;
  });

  after("Closing the connection", async function() {
    await api.disconnect();
  });

  describe("Picasso/Pablo Launch Plan - Phase 2A", function() {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    it("Users can not create a pablo pool.", async function() {
      const result = await createConsProdPool(api, composableManagerWallet, walletId1, baseAssetId, quoteAssetId, fee, baseWeight);
    });

    it("Create KSM/USDC Pool by root.", async function() {
      ksmUsdcPoolId = await createConsProdPool(api, composableManagerWallet, walletId1, baseAssetId, quoteAssetId, fee, baseWeight);
      //verify if the pool is created
      expect(poolId).to.be.a("number");
    });
  });

  describe("Picasso/Pablo Launch Plan - Phase 2B", function() {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    it("Wallet balance check should be >0", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();

      const balance = await Phase2.checkBalance(api, walletAlice.address);

      expect(balance.free.toBigInt() > 0).to.be.true; // .to.be.greater(0) didn't work for some reason.
    });
  });

  describe("Picasso/Pablo Launch Plan - Phase 2C", function() {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    it("Wallet balance check should be >0", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();

      const balance = await Phase2.checkBalance(api, walletAlice.address);

      expect(balance.free.toBigInt() > 0).to.be.true; // .to.be.greater(0) didn't work for some reason.
    });
  });

  describe("Picasso/Pablo Launch Plan - Phase 2D", function() {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    it("Wallet balance check should be >0", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();

      const balance = await Phase2.checkBalance(api, walletAlice.address);

      expect(balance.free.toBigInt() > 0).to.be.true; // .to.be.greater(0) didn't work for some reason.
    });
  });
});

/**
 * If the test file is quite small like this one, we often have the request functions within the same file.
 * Though for big files, like `txOracleTests.ts`, we outsource the tests handlers into an extra subdirectory
 * called `testHandlers`.
 */
export class Phase2 {
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
