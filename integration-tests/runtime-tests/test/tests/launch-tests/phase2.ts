import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { createLBPool, removeLiquidityFromPool } from "@composabletests/tests/pablo/testHandlers/pabloTestHelper";
import { mintAssetsToWallet, Pica } from "@composable/utils/mintingHelper";
import * as uniswap from "@composable/utils/pablo/uniswap";

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
// ToDo (D. Roth): Remove `SHORT` tag.
describe.only("[SHORT] Picasso/Pablo Launch Plan - Phase 2", function() {
  if (!testConfiguration.enabledTests.query.enabled) return;

  let api: ApiPromise;
  let sudoKey: KeyringPair,
    composableManagerWallet: KeyringPair,
    liquidityProviderWallet1: KeyringPair;
  let ksmUsdcPoolId: number,
    picaLBPPoolId: number;
  const picaAssetId = 1,
    ksmAssetId = 4,
    usdcAssetId = 131;
  const baseAmount = 250000000000;
  const quoteAmount = 250000000000;
  const minMintAmount = 0;

  before("Setting up the tests", async function() {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    const { devWalletAlice } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    composableManagerWallet = devWalletAlice;
    liquidityProviderWallet1 = devWalletAlice.derive("/test/launch/lp1");
  });

  before("Minting assets", async function() {
    this.timeout(5 * 60 * 1000);
    await mintAssetsToWallet(api, composableManagerWallet, sudoKey, [1, ksmAssetId, usdcAssetId]);
    await mintAssetsToWallet(api, liquidityProviderWallet1, sudoKey, [1, ksmAssetId, usdcAssetId]);
  });

  after("Closing the connection", async function() {
    await api.disconnect();
  });

  /**
   * 2A. Seed KSM/USDC pool
   *  - Pool config: 50/50 Uniswap AMM w/ 0.15% fee.
   *  - Tests add/remove liquidity to/from the pool by users.
   *  - Tests stake/unstake LP tokens by users.
   *  - Tests pool receiving farming rewards.
   *  - Tests trading fees & distribution.
   */
  describe("Picasso/Pablo Launch Plan - Phase 2A", function() {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    describe("Test 2A pool creation", function() {
      it("Users can not create a pablo pool.", async function() {
      });

      it("Create KSM/USDC Pool by root.", async function() {
        this.timeout(2 * 60 * 1000);

        const fee = 150000;
        const baseWeight = 500000;
        // ToDo (D. Roth): Update to use `root`.
        const { data: [result] } = await uniswap.sudo.sudoCreateMarket(
          api,
          sudoKey,
          composableManagerWallet.publicKey,
          ksmAssetId,
          usdcAssetId,
          fee,
          baseWeight
        );
        ksmUsdcPoolId = result.toNumber();
        //verify if the pool is created
        expect(ksmUsdcPoolId).to.be.a("number");
      });
    });

    describe("Test 2A pool liquidity", function() {
      describe("Test 2A pool add liquidity", function() {
        it("Users can add liquidity to the pool", async function() {
          this.timeout(2 * 60 * 1000);
          const { data: [result] } = await uniswap.addLiquidity(api, liquidityProviderWallet1, ksmUsdcPoolId, baseAmount, quoteAmount, minMintAmount, true);
        });

        it("Pool owner can add liquidity to the pool", async function() {
          this.timeout(2 * 60 * 1000);
          const { data: [result] } = await uniswap.sudo.sudoAddLiquidity(api, sudoKey, ksmUsdcPoolId, baseAmount, quoteAmount, minMintAmount, true);
        });
      });

      describe("Test 2A pool remove liquidity", function() {
        it("Users can remove liquidity from the pool", async function() {
          this.timeout(2 * 60 * 1000);
          const result = await removeLiquidityFromPool(api, ksmUsdcPoolId, liquidityProviderWallet1, Pica(500));
          expect(result.resultBase.toBn()).to.be.bignumber.closeTo(result.resultQuote.toBn(), Pica(15).toString());
        });

        it("Pool owner can remove liquidity from the pool", async function() {
          this.timeout(2 * 60 * 1000);
          const result = await removeLiquidityFromPool(api, ksmUsdcPoolId, composableManagerWallet, Pica(500));
          expect(result.resultBase.toBn()).to.be.bignumber.closeTo(result.resultQuote.toBn(), Pica(15).toString());
        });
      });
    });

    describe("Test 2A pool stake", function() {
      describe("Test 2A pool stake", function() {

      });

      describe("Test 2A pool unstake", function() {

      });
    });

    describe("Test 2A pool farming rewards", function() {

    });
  });

  /**
   * 2B. Launch PICA via LBP event
   *  - Pool consists of USDC only.
   *  - Pool starts 98/2, finishing at 50/50.
   */
  describe("Picasso/Pablo Launch Plan - Phase 2B", function() {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    it("Create PICA LBP w/ USDC", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      it("User1 can create a LBP", async function() {
        const currentBlock = await api.query.system.number();

        const result = await createLBPool(
          api,
          composableManagerWallet,
          composableManagerWallet,
          picaAssetId,
          usdcAssetId,
          currentBlock.toNumber() + 5,
          currentBlock.toNumber() + 50,
          980000,
          500000,
          0,
          0,
          0
        );
        picaLBPPoolId = result.resultPoolId;
        expect(picaLBPPoolId).to.be.a("number");
      });
    });
  });

  /**
   * 2C. Seed PICA/USDC pool
   *  - Pool config: 50/50 Uniswap AMM w/ 0.2% fee.
   *  - KSDM/USDC remains unchanged.
   *  - Pool receives additional PBLO farming rewards.
   *  - PICA/KSM will be created.
   */
  describe("Picasso/Pablo Launch Plan - Phase 2C", function() {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    it("Wallet balance check should be >0", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();

    });
  });

  /**
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
  describe("Picasso/Pablo Launch Plan - Phase 2D", function() {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    it("Wallet balance check should be >0", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();

    });
  });
});


export class Phase2 {

  public static async verifyPoolCreation() {

  }

  public static async verifyPoolLiquidityAdded() {

  }

  public static async verifyPoolLiquidityRemoved() {

  }
}
