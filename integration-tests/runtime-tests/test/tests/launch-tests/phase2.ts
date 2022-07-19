import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import * as pablo from "@composable/utils/pablo";
import { Phase2 } from "@composabletests/tests/launch-tests/testHelper";
import BN from "bn.js";

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
describe.only("[SHORT] Picasso/Pablo Launch Plan - Phase 2", function () {
  if (!testConfiguration.enabledTests.query.enabled) return;

  let api: ApiPromise;
  let sudoKey: KeyringPair,
    composableManagerWallet: KeyringPair,
    liquidityProviderWallet1: KeyringPair,
    traderWallet1: KeyringPair;
  let ksmUsdcLpTokenId: BN, picaUsdcLpTokenId: BN, picaKsmLpTokenId: BN, usdcAusdLpTokenId: BN;
  let ksmUsdcPoolId: BN, picaLBPPoolId: BN, picaUsdcPoolId: BN, picaKsmPoolId: BN, usdcAusdPoolId: BN;
  const picaAssetId = 1,
    ksmAssetId = 4,
    usdcAssetId = 131,
    btcAssetId = 1000,
    wethAssetId = 1111, // ToDo: Update to wETH assetId.
    ausdAssetId = 1112, // ToDo: Update to aUSD assetId.
    usdtAssetId = 2000; // ToDo: Update to USDT assetId.
  const baseAmount = 250000000000000000n;
  const quoteAmount = 250000000000000000n;
  const minMintAmount = 0;

  before("Setting up the tests", async function () {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    const { devWalletAlice } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    composableManagerWallet = devWalletAlice;
    liquidityProviderWallet1 = devWalletAlice.derive("/test/launch/lp1");
    traderWallet1 = devWalletAlice.derive("/test/launch/trader1");
  });

  before("Minting assets", async function () {
    this.timeout(5 * 60 * 1000);
    await mintAssetsToWallet(api, composableManagerWallet, sudoKey, [1, ksmAssetId, usdcAssetId]);
    await mintAssetsToWallet(api, liquidityProviderWallet1, sudoKey, [1, ksmAssetId, usdcAssetId]);
    await mintAssetsToWallet(api, traderWallet1, sudoKey, [1, ksmAssetId, usdcAssetId]);
  });

  after("Closing the connection", async function () {
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
  describe("Picasso/Pablo Launch Plan - Phase 2A", function () {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    describe("Test 2A pool creation", function () {
      it("Users can not create a pablo pool.", async function () {
        this.timeout(2 * 60 * 1000);

        const fee = 150000;
        const baseWeight = 500000;
        const baseAsset = ksmAssetId;
        const quoteAsset = usdcAssetId;
        const {
          data: [result]
        } = await pablo.uniswap.createMarket(
          api,
          sudoKey,
          composableManagerWallet.publicKey,
          baseAsset,
          quoteAsset,
          fee,
          baseWeight
        );
        // ToDo: Update to expect error!
        const { poolId, lpTokenId } = await Phase2.verifyLastPoolCreation(
          api,
          api.createType("PalletPabloPoolConfiguration", {
            ConstantProduct: {
              owner: composableManagerWallet.publicKey,
              pair: {
                base: baseAsset,
                quote: quoteAsset
              },
              lpToken: 100_000_000_000n,
              feeConfig: {
                feeRate: fee,
                ownerFeeRate: 200000,
                protocolFeeRate: 1000000
              },
              baseWeight: baseWeight,
              quoteWeight: baseWeight
            }
          })
        );
        ksmUsdcPoolId = poolId;
        ksmUsdcLpTokenId = lpTokenId;
      });

      it("Create KSM/USDC uniswap pool by root.", async function () {
        // ToDo: Update when root can create pools!
        this.skip();
        this.timeout(2 * 60 * 1000);

        const fee = 150000;
        const baseWeight = 500000;
        const baseAsset = ksmAssetId;
        const quoteAsset = usdcAssetId;

        const {
          data: [result]
        } = await pablo.uniswap.sudo.sudoCreateMarket(
          api,
          sudoKey,
          composableManagerWallet.publicKey,
          baseAsset,
          quoteAsset,
          fee,
          baseWeight
        );
        expect(result.isOk).to.be.true;
      });
    });

    describe("Test 2A pool liquidity", function () {
      describe("Test 2A pool add liquidity", function () {
        it("Users can add liquidity to the pool", async function () {
          this.timeout(2 * 60 * 1000);
          const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
            ksmUsdcLpTokenId.toString(),
            liquidityProviderWallet1.publicKey
          );
          const {
            data: [result]
          } = await pablo.addLiquidity(
            api,
            liquidityProviderWallet1,
            ksmUsdcPoolId,
            baseAmount,
            quoteAmount,
            minMintAmount,
            true
          );
          const lpTokenBalanceAfter = await api.rpc.assets.balanceOf(
            ksmUsdcLpTokenId.toString(),
            liquidityProviderWallet1.publicKey
          );
          expect(new BN(lpTokenBalanceAfter.toString())).to.be.bignumber.greaterThan(
            new BN(lpTokenBalanceBefore.toString())
          );
        });

        it("Pool owner (root) can add liquidity to the pool", async function () {
          // ToDo: Update when root can create pools!
          this.skip();
          this.timeout(2 * 60 * 1000);
          const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
            ksmUsdcLpTokenId.toString(),
            liquidityProviderWallet1.publicKey
          );
          const {
            data: [result]
          } = await pablo.sudo.sudoAddLiquidity(
            api,
            sudoKey,
            ksmUsdcPoolId,
            baseAmount,
            quoteAmount,
            minMintAmount,
            true
          );
          const lpTokenBalanceAfter = await api.rpc.assets.balanceOf(
            ksmUsdcLpTokenId.toString(),
            liquidityProviderWallet1.publicKey
          );
          expect(new BN(lpTokenBalanceAfter.toString())).to.be.bignumber.greaterThan(
            new BN(lpTokenBalanceBefore.toString())
          );
        });
      });

      describe("Test 2A pool remove liquidity", function () {
        it("Users can remove liquidity from the pool", async function () {
          this.timeout(2 * 60 * 1000);
          const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
            ksmUsdcLpTokenId.toString(),
            liquidityProviderWallet1.publicKey
          );
          const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
          const baseAmount = 0;
          const quoteAmount = 0;
          const {
            data: [result]
          } = await pablo.removeLiquidity(
            api,
            liquidityProviderWallet1,
            ksmUsdcPoolId,
            lpAmount,
            baseAmount,
            quoteAmount
          );
          const lpTokenBalanceAfter = await api.rpc.assets.balanceOf(
            ksmUsdcLpTokenId.toString(),
            liquidityProviderWallet1.publicKey
          );
          expect(new BN(lpTokenBalanceAfter.toString())).to.be.bignumber.lessThan(
            new BN(lpTokenBalanceBefore.toString())
          );
        });
        it("Pool owner (sudo) can remove liquidity from the pool", async function () {
          // ToDo: Update when root can create pools!
          this.skip();
          this.timeout(2 * 60 * 1000);
          const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
            ksmUsdcLpTokenId.toString(),
            liquidityProviderWallet1.publicKey
          );
          const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
          const baseAmount = 0;
          const quoteAmount = 0;
          const {
            data: [result]
          } = await pablo.sudo.sudoRemoveLiquidity(api, sudoKey, ksmUsdcPoolId, lpAmount, baseAmount, quoteAmount);
          const lpTokenBalanceAfter = await api.rpc.assets.balanceOf(
            ksmUsdcLpTokenId.toString(),
            liquidityProviderWallet1.publicKey
          );
          expect(new BN(lpTokenBalanceAfter.toString())).to.be.bignumber.lessThan(
            new BN(lpTokenBalanceBefore.toString())
          );
        });
      });
    });

    describe("Test 2A trading", function () {
      describe("Test 2A buy", function () {
        it("Users can buy from pool", async function () {
          this.timeout(2 * 60 * 1000);
          const assetIdToBuy = ksmAssetId;
          const amount = 100_000_000_000n;
          const minReceive = 0;
          const keepAlive = true;
          const {
            data: [result]
          } = await pablo.buyTokens(
            api,
            liquidityProviderWallet1,
            ksmUsdcPoolId,
            assetIdToBuy,
            amount,
            minReceive,
            keepAlive
          );
        });
      });

      describe("Test 2A sell", function () {
        it("Users can sell to pool", async function () {
          this.timeout(2 * 60 * 1000);
          const assetIdToSell = ksmAssetId;
          const amount = 100_000_000_000n;
          const minReceive = 0;
          const keepAlive = true;
          const {
            data: [result]
          } = await pablo.sellTokens(
            api,
            liquidityProviderWallet1,
            ksmUsdcPoolId,
            assetIdToSell,
            amount,
            minReceive,
            keepAlive
          );
        });
      });

      describe("Test 2A swap", function () {
        it("Users can swap in the pool", async function () {
          this.timeout(2 * 60 * 1000);
          const pair = { base: ksmAssetId, quote: usdcAssetId };
          const amount = 100_000_000_000n;
          const minReceive = 0;
          const keepAlive = true;
          const {
            data: [result]
          } = await pablo.swapTokens(api, liquidityProviderWallet1, ksmUsdcPoolId, pair, amount, minReceive, keepAlive);
        });
      });
    });

    describe("Test 2A pool stake", function () {
      describe("Test 2A pool stake", function () {
        it("Users can stake LP tokens", async function () {
          // ToDo: Implement when pablo staking is done.
        });
      });

      describe("Test 2A pool unstake", function () {
        it("Users can unstake LP tokens", async function () {
          // ToDo: Implement when pablo staking is done.
        });
      });
    });

    describe("Test 2A pool farming rewards", function () {
      // ToDo: Implement when pablo staking is done.
    });
  });

  /**
   * 2B. Launch PICA via LBP event
   *  - Pool consists of USDC only.
   *  - Pool starts 98/2, finishing at 50/50.
   */
  describe("Picasso/Pablo Launch Plan - Phase 2B", function () {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    it("Create PICA LBP w/ USDC", async function () {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);
      const currentBlock = await api.query.system.number();
      const baseAsset = picaAssetId;
      const quoteAsset = usdcAssetId;
      const saleStart = currentBlock.toNumber() + 5;
      const saleEnd = currentBlock.toNumber() + 50;
      const initialWeight = 950000;
      const finalWeight = 500000;
      const feeRate = 0;
      const ownerFeeRate = 0;
      const protocolFeeRate = 0;
      // ToDo: Switch to sudo!
      const result = await pablo.liquidityBootstrapping.createMarket(
        api,
        sudoKey,
        composableManagerWallet.publicKey,
        picaAssetId,
        usdcAssetId,
        saleStart,
        saleEnd,
        initialWeight,
        finalWeight,
        feeRate,
        ownerFeeRate,
        protocolFeeRate
      );

      const { poolId, lpTokenId } = await Phase2.verifyLastPoolCreation(
        api,
        api.createType("PalletPabloPoolConfiguration", {
          LiquidityBootstrapping: {
            owner: api.createType("AccountId32", composableManagerWallet.publicKey),
            pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
              base: api.createType("u128", baseAsset),
              quote: api.createType("u128", quoteAsset)
            }),
            sale: api.createType("ComposableTraitsDexSale", {
              start: api.createType("u32", saleStart),
              end: api.createType("u32", saleEnd),
              initialWeight: api.createType("Permill", initialWeight),
              finalWeight: api.createType("Permill", finalWeight)
            }),
            feeConfig: api.createType("ComposableTraitsDexFeeConfig", {
              feeRate: api.createType("Permill", feeRate),
              ownerFeeRate: api.createType("Permill", ownerFeeRate),
              protocolFeeRate: api.createType("Permill", protocolFeeRate)
            })
          }
        })
      );
      picaLBPPoolId = poolId;
    });

    describe("Test 2B pool liquidity", function () {
      describe("Test 2B pool add liquidity", function () {
        it("Users can add liquidity to the pool", async function () {
          this.timeout(2 * 60 * 1000);
          const {
            data: [result]
          } = await pablo.addLiquidity(
            api,
            liquidityProviderWallet1,
            picaLBPPoolId,
            baseAmount,
            quoteAmount,
            minMintAmount,
            true
          );
        });

        it("Pool owner (root) can add liquidity to the pool", async function () {
          // ToDo: Update when root can create pools!
          this.skip();
          this.timeout(2 * 60 * 1000);
          const {
            data: [result]
          } = await pablo.sudo.sudoAddLiquidity(
            api,
            sudoKey,
            picaLBPPoolId,
            baseAmount,
            quoteAmount,
            minMintAmount,
            true
          );
        });
      });

      describe("Test 2B pool remove liquidity", function () {
        it("Users can remove liquidity from the pool", async function () {
          this.timeout(2 * 60 * 1000);
          const lpAmount = 100_000_000;
          const baseAmount = 0;
          const quoteAmount = 0;
          const {
            data: [result]
          } = await pablo.removeLiquidity(
            api,
            liquidityProviderWallet1,
            picaLBPPoolId,
            lpAmount,
            baseAmount,
            quoteAmount
          );
        });
        it("Pool owner (sudo) can remove liquidity from the pool", async function () {
          // ToDo: Update when root can create pools!
          this.skip();
          this.timeout(2 * 60 * 1000);
          const lpAmount = 100_000_000;
          const baseAmount = 0;
          const quoteAmount = 0;
          const {
            data: [result]
          } = await pablo.sudo.sudoRemoveLiquidity(api, sudoKey, picaLBPPoolId, lpAmount, baseAmount, quoteAmount);
        });
      });
    });

    describe("Test 2B trading", function () {
      describe("Test 2B buy", function () {
        it("Users can buy from pool", async function () {
          this.timeout(2 * 60 * 1000);
          const assetIdToBuy = picaAssetId;
          const amount = 100_000_000_000n;
          const minReceive = 0;
          const keepAlive = true;
          const {
            data: [result]
          } = await pablo.buyTokens(api, traderWallet1, picaLBPPoolId, assetIdToBuy, amount, minReceive, keepAlive);
        });
      });

      describe("Test 2B sell", function () {
        it("Users can sell to pool", async function () {
          this.timeout(2 * 60 * 1000);
          const assetIdToSell = picaAssetId;
          const amount = 50_000_000_000n;
          const minReceive = 0;
          const keepAlive = true;
          const {
            data: [result]
          } = await pablo.sellTokens(api, traderWallet1, picaLBPPoolId, assetIdToSell, amount, minReceive, keepAlive);
        });
      });

      describe("Test 2B swap", function () {
        it("Users can swap in the pool", async function () {
          this.timeout(2 * 60 * 1000);
          const pair = { base: picaAssetId, quote: usdcAssetId };
          const amount = 10_000_000_000n;
          const minReceive = 0;
          const keepAlive = true;
          const {
            data: [result]
          } = await pablo.swapTokens(api, traderWallet1, picaLBPPoolId, pair, amount, minReceive, keepAlive);
        });
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
  describe("Picasso/Pablo Launch Plan - Phase 2C", function () {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    describe("2C:1 PICA/USDC Uniswap Pool", function () {
      it("Create PICA/USDC uniswap pool", async function () {
        if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
        this.timeout(2 * 60 * 1000);
        const baseAsset = picaAssetId;
        const quoteAsset = usdcAssetId;
        const fee = 200000;
        const baseWeight = 500000;
        const {
          data: [result]
        } = await pablo.uniswap.createMarket(
          // ToDo: Switch to sudo!
          api,
          sudoKey,
          composableManagerWallet.publicKey,
          baseAsset,
          quoteAsset,
          fee,
          baseWeight
        );
        const { poolId, lpTokenId } = await Phase2.verifyLastPoolCreation(
          api,
          api.createType("PalletPabloPoolConfiguration", {
            ConstantProduct: {
              owner: composableManagerWallet.publicKey,
              pair: {
                base: baseAsset,
                quote: quoteAsset
              },
              lpToken: 100_000_000_000n,
              feeConfig: {
                feeRate: fee,
                ownerFeeRate: 200000,
                protocolFeeRate: 1000000
              },
              baseWeight: baseWeight,
              quoteWeight: baseWeight
            }
          })
        );
        picaUsdcPoolId = poolId;
        picaUsdcLpTokenId = lpTokenId;
      });

      describe("Test 2C:1 pool liquidity", function () {
        describe("Test 2C:1 pool add liquidity", function () {
          it("Users can add liquidity to the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              picaUsdcLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const {
              data: [result]
            } = await pablo.addLiquidity(
              api,
              liquidityProviderWallet1,
              picaUsdcPoolId,
              baseAmount,
              quoteAmount,
              minMintAmount,
              true
            );
            const lpTokenBalanceAfter = await api.rpc.assets.balanceOf(
              picaUsdcLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            expect(new BN(lpTokenBalanceAfter.toString())).to.be.bignumber.greaterThan(
              new BN(lpTokenBalanceBefore.toString())
            );
          });

          it("Pool owner (root) can add liquidity to the pool", async function () {
            // ToDo: Update when root can create pools!
            this.skip();
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              picaUsdcLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const {
              data: [result]
            } = await pablo.sudo.sudoAddLiquidity(
              api,
              sudoKey,
              picaUsdcPoolId,
              baseAmount,
              quoteAmount,
              minMintAmount,
              true
            );
            const lpTokenBalanceAfter = await api.rpc.assets.balanceOf(
              picaUsdcLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            expect(new BN(lpTokenBalanceAfter.toString())).to.be.bignumber.greaterThan(
              new BN(lpTokenBalanceBefore.toString())
            );
          });
        });

        describe("Test 2C:1 pool remove liquidity", function () {
          it("Users can remove liquidity from the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              picaUsdcLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
            const baseAmount = 0;
            const quoteAmount = 0;
            const {
              data: [result]
            } = await pablo.removeLiquidity(
              api,
              liquidityProviderWallet1,
              picaUsdcPoolId,
              lpAmount,
              baseAmount,
              quoteAmount
            );
            const lpTokenBalanceAfter = await api.rpc.assets.balanceOf(
              picaUsdcLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            expect(new BN(lpTokenBalanceAfter.toString())).to.be.bignumber.lessThan(
              new BN(lpTokenBalanceBefore.toString())
            );
          });
          it("Pool owner (sudo) can remove liquidity from the pool", async function () {
            // ToDo: Update when root can create pools!
            this.skip();
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              picaUsdcLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
            const baseAmount = 0;
            const quoteAmount = 0;
            const {
              data: [result]
            } = await pablo.sudo.sudoRemoveLiquidity(api, sudoKey, picaUsdcPoolId, lpAmount, baseAmount, quoteAmount);
            const lpTokenBalanceAfter = await api.rpc.assets.balanceOf(
              picaUsdcLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            expect(new BN(lpTokenBalanceAfter.toString())).to.be.bignumber.lessThan(
              new BN(lpTokenBalanceBefore.toString())
            );
          });
        });
      });

      describe("Test 2C:1 trading", function () {
        describe("Test 2C:1 buy", function () {
          it("Users can buy from pool", async function () {
            this.timeout(2 * 60 * 1000);
            const assetIdToBuy = picaAssetId;
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.buyTokens(
              api,
              liquidityProviderWallet1,
              picaUsdcPoolId,
              assetIdToBuy,
              amount,
              minReceive,
              keepAlive
            );
          });
        });

        describe("Test 2C:1 sell", function () {
          it("Users can sell to pool", async function () {
            this.timeout(2 * 60 * 1000);
            const assetIdToSell = picaAssetId;
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.sellTokens(
              api,
              liquidityProviderWallet1,
              picaUsdcPoolId,
              assetIdToSell,
              amount,
              minReceive,
              keepAlive
            );
          });
        });

        describe("Test 2C:1 swap", function () {
          it("Users can swap in the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const pair = { base: picaAssetId, quote: usdcAssetId };
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.swapTokens(
              api,
              liquidityProviderWallet1,
              picaUsdcPoolId,
              pair,
              amount,
              minReceive,
              keepAlive
            );
          });
        });
      });
    });

    describe("2C: PICA/KSM Uniswap Pool", function () {
      it("Create PICA/KSM pool", async function () {
        if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
        this.timeout(2 * 60 * 1000);

        const fee = 200000;
        const baseWeight = 500000;
        const baseAsset = picaAssetId;
        const quoteAsset = ksmAssetId;
        const {
          data: [result]
        } = await pablo.uniswap.createMarket(
          // ToDo: Switch to sudo!
          api,
          sudoKey,
          composableManagerWallet.publicKey,
          baseAsset,
          quoteAsset,
          fee,
          baseWeight
        );
        const { poolId, lpTokenId } = await Phase2.verifyLastPoolCreation(
          api,
          api.createType("PalletPabloPoolConfiguration", {
            ConstantProduct: {
              owner: composableManagerWallet.publicKey,
              pair: {
                base: baseAsset,
                quote: quoteAsset
              },
              lpToken: 100_000_000_000n,
              feeConfig: {
                feeRate: fee,
                ownerFeeRate: 200000,
                protocolFeeRate: 1000000
              },
              baseWeight: baseWeight,
              quoteWeight: baseWeight
            }
          })
        );
        picaKsmPoolId = poolId;
        picaKsmLpTokenId = lpTokenId;
      });

      describe("Test 2C:2 pool liquidity", function () {
        describe("Test 2C:2 pool add liquidity", function () {
          it("Users can add liquidity to the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              picaKsmLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const {
              data: [result]
            } = await pablo.addLiquidity(
              api,
              liquidityProviderWallet1,
              picaKsmPoolId,
              baseAmount,
              quoteAmount,
              minMintAmount,
              true
            );
            const lpTokenBalanceAfter = await api.rpc.assets.balanceOf(
              picaKsmLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            expect(new BN(lpTokenBalanceAfter.toString())).to.be.bignumber.greaterThan(
              new BN(lpTokenBalanceBefore.toString())
            );
          });

          it("Pool owner (root) can add liquidity to the pool", async function () {
            // ToDo: Update when root can create pools!
            this.skip();
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              picaKsmLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const {
              data: [result]
            } = await pablo.sudo.sudoAddLiquidity(
              api,
              sudoKey,
              picaKsmPoolId,
              baseAmount,
              quoteAmount,
              minMintAmount,
              true
            );
            const lpTokenBalanceAfter = await api.rpc.assets.balanceOf(
              picaKsmLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            expect(new BN(lpTokenBalanceAfter.toString())).to.be.bignumber.greaterThan(
              new BN(lpTokenBalanceBefore.toString())
            );
          });
        });

        describe("Test 2C pool remove liquidity", function () {
          it("Users can remove liquidity from the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              picaKsmLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
            const baseAmount = 0;
            const quoteAmount = 0;
            const {
              data: [result]
            } = await pablo.removeLiquidity(
              api,
              liquidityProviderWallet1,
              picaKsmPoolId,
              lpAmount,
              baseAmount,
              quoteAmount
            );
            const lpTokenBalanceAfter = await api.rpc.assets.balanceOf(
              picaKsmLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            expect(new BN(lpTokenBalanceAfter.toString())).to.be.bignumber.lessThan(
              new BN(lpTokenBalanceBefore.toString())
            );
          });
          it("Pool owner (sudo) can remove liquidity from the pool", async function () {
            // ToDo: Update when root can create pools!
            this.skip();
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              picaKsmLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
            const baseAmount = 0;
            const quoteAmount = 0;
            const {
              data: [result]
            } = await pablo.sudo.sudoRemoveLiquidity(api, sudoKey, picaKsmPoolId, lpAmount, baseAmount, quoteAmount);
            const lpTokenBalanceAfter = await api.rpc.assets.balanceOf(
              picaKsmLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            expect(new BN(lpTokenBalanceAfter.toString())).to.be.bignumber.lessThan(
              new BN(lpTokenBalanceBefore.toString())
            );
          });
        });
      });

      describe("Test 2C:2 trading", function () {
        describe("Test 2C:2 buy", function () {
          it("Users can buy from pool", async function () {
            this.timeout(2 * 60 * 1000);
            const assetIdToBuy = picaAssetId;
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.buyTokens(
              api,
              liquidityProviderWallet1,
              picaKsmPoolId,
              assetIdToBuy,
              amount,
              minReceive,
              keepAlive
            );
          });
        });

        describe("Test 2C:2 sell", function () {
          it("Users can sell to pool", async function () {
            this.timeout(2 * 60 * 1000);
            const assetIdToSell = picaAssetId;
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.sellTokens(
              api,
              liquidityProviderWallet1,
              picaKsmPoolId,
              assetIdToSell,
              amount,
              minReceive,
              keepAlive
            );
          });
        });

        describe("Test 2C:2 swap", function () {
          it("Users can swap in the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const pair = { base: picaAssetId, quote: ksmAssetId };
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.swapTokens(
              api,
              liquidityProviderWallet1,
              picaKsmPoolId,
              pair,
              amount,
              minReceive,
              keepAlive
            );
          });
        });
      });
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
  describe("Picasso/Pablo Launch Plan - Phase 2D", function () {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    describe("2D:1 USDC/aUSD StableSwap Pool", function () {
      it("Create USDC/aUSD stableswap pool", async function () {
        if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
        this.timeout(2 * 60 * 1000);
        const amplificationCoefficient = 24; // ToDo: Update!
        const fee = 100000; // ToDo: Update!
        const baseAsset = usdcAssetId;
        const quoteAsset = ausdAssetId;
        const {
          data: [result]
        } = await pablo.stableswap.createMarket(
          // ToDo: Switch to sudo!
          api,
          sudoKey,
          composableManagerWallet.publicKey,
          baseAsset,
          quoteAsset,
          amplificationCoefficient,
          fee
        );
        const { poolId, lpTokenId } = await Phase2.verifyLastPoolCreation(
          api,
          api.createType("PalletPabloPoolConfiguration", {
            StableSwap: {
              owner: composableManagerWallet.publicKey,
              pair: {
                base: baseAsset,
                quote: quoteAsset
              },
              amplificationCoefficient: amplificationCoefficient,
              lpToken: 100_000_000_000n,
              fee: fee
            }
          })
        );
        usdcAusdPoolId = poolId;
        usdcAusdLpTokenId = lpTokenId;
      });

      describe("Test 2D:1 pool liquidity", function () {
        describe("Test 2D:1 pool add liquidity", function () {
          it("Users can add liquidity to the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAusdLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const {
              data: [result]
            } = await pablo.addLiquidity(
              api,
              liquidityProviderWallet1,
              usdcAusdPoolId,
              baseAmount,
              quoteAmount,
              minMintAmount,
              true
            );
            const lpTokenBalanceAfter = await api.rpc.assets.balanceOf(
              usdcAusdLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            expect(new BN(lpTokenBalanceAfter.toString())).to.be.bignumber.greaterThan(
              new BN(lpTokenBalanceBefore.toString())
            );
          });

          it("Pool owner (root) can add liquidity to the pool", async function () {
            // ToDo: Update when root can create pools!
            this.skip();
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAusdLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const {
              data: [result]
            } = await pablo.sudo.sudoAddLiquidity(
              api,
              sudoKey,
              usdcAusdPoolId,
              baseAmount,
              quoteAmount,
              minMintAmount,
              true
            );
            const lpTokenBalanceAfter = await api.rpc.assets.balanceOf(
              usdcAusdLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            expect(new BN(lpTokenBalanceAfter.toString())).to.be.bignumber.greaterThan(
              new BN(lpTokenBalanceBefore.toString())
            );
          });
        });

        describe("Test 2C pool remove liquidity", function () {
          it("Users can remove liquidity from the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAusdLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
            const baseAmount = 0;
            const quoteAmount = 0;
            const {
              data: [result]
            } = await pablo.removeLiquidity(
              api,
              liquidityProviderWallet1,
              usdcAusdPoolId,
              lpAmount,
              baseAmount,
              quoteAmount
            );
            const lpTokenBalanceAfter = await api.rpc.assets.balanceOf(
              usdcAusdLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            expect(new BN(lpTokenBalanceAfter.toString())).to.be.bignumber.lessThan(
              new BN(lpTokenBalanceBefore.toString())
            );
          });
          it("Pool owner (sudo) can remove liquidity from the pool", async function () {
            // ToDo: Update when root can create pools!
            this.skip();
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAusdLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
            const baseAmount = 0;
            const quoteAmount = 0;
            const {
              data: [result]
            } = await pablo.sudo.sudoRemoveLiquidity(api, sudoKey, usdcAusdPoolId, lpAmount, baseAmount, quoteAmount);
            const lpTokenBalanceAfter = await api.rpc.assets.balanceOf(
              usdcAusdLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            expect(new BN(lpTokenBalanceAfter.toString())).to.be.bignumber.lessThan(
              new BN(lpTokenBalanceBefore.toString())
            );
          });
        });
      });

      describe("Test 2C:2 trading", function () {
        describe("Test 2C:2 buy", function () {
          it("Users can buy from pool", async function () {
            this.timeout(2 * 60 * 1000);
            const assetIdToBuy = usdcAssetId;
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.buyTokens(
              api,
              liquidityProviderWallet1,
              usdcAusdPoolId,
              assetIdToBuy,
              amount,
              minReceive,
              keepAlive
            );
          });
        });

        describe("Test 2C:2 sell", function () {
          it("Users can sell to pool", async function () {
            this.timeout(2 * 60 * 1000);
            const assetIdToSell = usdcAssetId;
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.sellTokens(
              api,
              liquidityProviderWallet1,
              usdcAusdPoolId,
              assetIdToSell,
              amount,
              minReceive,
              keepAlive
            );
          });
        });

        describe("Test 2C:2 swap", function () {
          it("Users can swap in the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const pair = { base: usdcAssetId, quote: ausdAssetId };
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.swapTokens(
              api,
              liquidityProviderWallet1,
              usdcAusdPoolId,
              pair,
              amount,
              minReceive,
              keepAlive
            );
          });
        });
      });
    });

    describe("2D:2 wETH/KSM StableSwap Pool", function () {
      it("Create wETH/KSM uniswap pool", async function () {
        if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
        this.timeout(2 * 60 * 1000);

        const fee = 150000;
        const baseWeight = 500000;
        const baseAsset = wethAssetId;
        const quoteAsset = ksmAssetId;
        const {
          data: [result]
        } = await pablo.uniswap.createMarket(
          // ToDo: Switch to sudo!
          api,
          sudoKey,
          composableManagerWallet.publicKey,
          baseAsset,
          quoteAsset,
          fee,
          baseWeight
        );
        const { poolId, lpTokenId } = await Phase2.verifyLastPoolCreation(
          api,
          api.createType("PalletPabloPoolConfiguration", {
            ConstantProduct: {
              owner: composableManagerWallet.publicKey,
              pair: {
                base: baseAsset,
                quote: quoteAsset
              },
              lpToken: 100_000_000_000n,
              feeConfig: {
                feeRate: fee,
                ownerFeeRate: 200000,
                protocolFeeRate: 1000000
              },
              baseWeight: baseWeight,
              quoteWeight: baseWeight
            }
          })
        );
      });
    });

    describe("2D:3 wBTC/KSM Uniswap Pool", function () {
      it("Create wBTC/KSM uniswap pool", async function () {
        if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
        this.timeout(2 * 60 * 1000);
        const fee = 150000;
        const baseWeight = 500000;
        const baseAsset = btcAssetId;
        const quoteAsset = ksmAssetId;
        const {
          data: [result]
        } = await pablo.uniswap.createMarket(
          // ToDo: Switch to sudo!
          api,
          sudoKey,
          composableManagerWallet.publicKey,
          baseAsset,
          quoteAsset,
          fee,
          baseWeight
        );
        const { poolId, lpTokenId } = await Phase2.verifyLastPoolCreation(
          api,
          api.createType("PalletPabloPoolConfiguration", {
            ConstantProduct: {
              owner: composableManagerWallet.publicKey,
              pair: {
                base: baseAsset,
                quote: quoteAsset
              },
              lpToken: 100_000_000_000n,
              feeConfig: {
                feeRate: fee,
                ownerFeeRate: 200000,
                protocolFeeRate: 1000000
              },
              baseWeight: baseWeight,
              quoteWeight: baseWeight
            }
          })
        );
      });
    });

    describe("2D:4 USDC/USDT StableSwap Pool", function () {
      it("Create USDC/USDT stableswap pool", async function () {
        if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
        this.timeout(2 * 60 * 1000);

        const amplificationCoefficient = 24; // ToDo: Update!
        const fee = 100000; // ToDo: Update!
        const baseAsset = usdcAssetId;
        const quoteAsset = usdtAssetId;
        const {
          data: [result]
        } = await pablo.stableswap.createMarket(
          // ToDo: Switch to sudo!
          api,
          sudoKey,
          composableManagerWallet.publicKey,
          baseAsset,
          quoteAsset,
          amplificationCoefficient,
          fee
        );
        const { poolId, lpTokenId } = await Phase2.verifyLastPoolCreation(
          api,
          api.createType("PalletPabloPoolConfiguration", {
            StableSwap: {
              owner: composableManagerWallet.publicKey,
              pair: {
                base: baseAsset,
                quote: quoteAsset
              },
              amplificationCoefficient: amplificationCoefficient,
              lpToken: 100_000_000_000n,
              fee: fee
            }
          })
        );
      });
    });
  });
});
