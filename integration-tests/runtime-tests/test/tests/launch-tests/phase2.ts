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
describe.only("[SHORT][LAUNCH2] Picasso/Pablo Launch Plan - Phase 2", function () {
  if (!testConfiguration.enabledTests.query.enabled) return;
  this.bail(true);

  let api: ApiPromise;
  let sudoKey: KeyringPair,
    composableManagerWallet: KeyringPair,
    liquidityProviderWallet1: KeyringPair,
    liquidityProviderWallet2: KeyringPair,
    traderWallet1: KeyringPair;
  let ksmUsdcLpTokenId: BN,
    picaUsdcLpTokenId: BN,
    picaKsmLpTokenId: BN,
    usdcAusdLpTokenId: BN,
    wethKsmLpTokenId: BN,
    wbtcKsmLpTokenId: BN,
    usdcUsdtLpTokenId: BN;
  let ksmUsdcPoolId: BN,
    picaLBPPoolId: BN,
    picaUsdcPoolId: BN,
    picaKsmPoolId: BN,
    usdcAusdPoolId: BN,
    wethKsmPoolId: BN,
    wbtcKsmPoolId: BN,
    usdcUsdtPoolId: BN;
  const picaAssetId = 1,
    ksmAssetId = 4,
    usdcAssetId = 131,
    btcAssetId = 1000, // ToDo: Update to wBTC assetId.
    wethAssetId = 1111, // ToDo: Update to wETH assetId.
    ausdAssetId = 1112, // ToDo: Update to aUSD assetId.
    usdtAssetId = 2000; // ToDo: Update to USDT assetId.
  const baseAmount = 250_000_000_000_000_000n;
  const quoteAmount = 250_000_000_000_000_000n;
  const minMintAmount = 0;

  before("Setting up the tests", async function () {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    const { devWalletAlice } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    composableManagerWallet = devWalletAlice;
    liquidityProviderWallet1 = devWalletAlice.derive("/test/launch/lp1");
    liquidityProviderWallet2 = devWalletAlice.derive("/test/launch/lp12");
    traderWallet1 = devWalletAlice.derive("/test/launch/trader1");
  });

  before("Minting assets", async function () {
    this.timeout(5 * 60 * 1000);
    const assetIdList = [1, ksmAssetId, usdcAssetId, wethAssetId, btcAssetId, usdtAssetId, ausdAssetId];
    await mintAssetsToWallet(api, composableManagerWallet, sudoKey, assetIdList);
    await mintAssetsToWallet(api, liquidityProviderWallet1, sudoKey, assetIdList);
    await mintAssetsToWallet(api, liquidityProviderWallet2, sudoKey, assetIdList);
    await mintAssetsToWallet(api, traderWallet1, sudoKey, assetIdList);
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
          sudoKey, // ToDo: Update to use different wallet!
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
          const ksmBalanceBefore = await api.rpc.assets.balanceOf(
            ksmAssetId.toString(),
            liquidityProviderWallet1.publicKey
          );

          const usdcBalanceBefore = await api.rpc.assets.balanceOf(
            usdcAssetId.toString(),
            liquidityProviderWallet1.publicKey
          );
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
          await Phase2.verifyPoolLiquidityAdded(
            api,
            ksmAssetId,
            usdcAssetId,
            ksmUsdcLpTokenId,
            liquidityProviderWallet1.publicKey,
            baseAmount,
            new BN(ksmBalanceBefore.toString()),
            new BN(usdcBalanceBefore.toString()),
            new BN(lpTokenBalanceBefore.toString())
          );
        });

        it("Pool owner can add liquidity to the pool", async function () {
          // ToDo: Update when root can create pools!
          this.skip();
          this.timeout(2 * 60 * 1000);
          const ksmBalanceBefore = await api.rpc.assets.balanceOf(
            ksmAssetId.toString(),
            liquidityProviderWallet1.publicKey
          );

          const usdcBalanceBefore = await api.rpc.assets.balanceOf(
            usdcAssetId.toString(),
            liquidityProviderWallet1.publicKey
          );
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
          await Phase2.verifyPoolLiquidityAdded(
            api,
            ksmAssetId,
            usdcAssetId,
            ksmUsdcLpTokenId,
            liquidityProviderWallet1.publicKey,
            baseAmount,
            new BN(ksmBalanceBefore.toString()),
            new BN(usdcBalanceBefore.toString()),
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
          const ksmBalanceBefore = await api.rpc.assets.balanceOf(
            ksmAssetId.toString(),
            liquidityProviderWallet1.publicKey
          );
          const usdcBalanceBefore = await api.rpc.assets.balanceOf(
            usdcAssetId.toString(),
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
          await Phase2.verifyPoolLiquidityRemoved(
            api,
            ksmAssetId,
            usdcAssetId,
            ksmUsdcLpTokenId,
            liquidityProviderWallet1.publicKey,
            baseAmount,
            new BN(ksmBalanceBefore.toString()),
            new BN(usdcBalanceBefore.toString()),
            new BN(lpTokenBalanceBefore.toString())
          );
        });
        it("Pool owner can remove liquidity from the pool", async function () {
          // ToDo: Update when root can create pools!
          this.skip();
          this.timeout(2 * 60 * 1000);
          const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
            ksmUsdcLpTokenId.toString(),
            liquidityProviderWallet1.publicKey
          );
          const ksmBalanceBefore = await api.rpc.assets.balanceOf(
            ksmAssetId.toString(),
            liquidityProviderWallet1.publicKey
          );
          const usdcBalanceBefore = await api.rpc.assets.balanceOf(
            usdcAssetId.toString(),
            liquidityProviderWallet1.publicKey
          );
          const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
          const baseAmount = 0;
          const quoteAmount = 0;
          const {
            data: [result]
          } = await pablo.removeLiquidity(
            api,
            composableManagerWallet,
            ksmUsdcPoolId,
            lpAmount,
            baseAmount,
            quoteAmount
          );
          await Phase2.verifyPoolLiquidityRemoved(
            api,
            ksmAssetId,
            usdcAssetId,
            ksmUsdcLpTokenId,
            composableManagerWallet.publicKey,
            baseAmount,
            new BN(ksmBalanceBefore.toString()),
            new BN(usdcBalanceBefore.toString()),
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
          } = await pablo.buyTokens(api, traderWallet1, ksmUsdcPoolId, assetIdToBuy, amount, minReceive, keepAlive);
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
          } = await pablo.sellTokens(api, traderWallet1, ksmUsdcPoolId, assetIdToSell, amount, minReceive, keepAlive);
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
          } = await pablo.swapTokens(api, traderWallet1, ksmUsdcPoolId, pair, amount, minReceive, keepAlive);
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
      const saleStart = currentBlock.toNumber() + 25;
      const saleEnd = currentBlock.toNumber() + 1000;
      // Todo: Set initial weight to 980000.
      const initialWeight = 950000;
      const finalWeight = 500000;
      const feeRate = 0;
      const ownerFeeRate = 0;
      const protocolFeeRate = 0;
      // ToDo: Switch to sudo!
      const result = await pablo.liquidityBootstrapping.createMarket(
        api,
        composableManagerWallet,
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

      const { poolId } = await Phase2.verifyLastPoolCreation(
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
        it("Users can not add liquidity to the pool", async function () {
          this.timeout(2 * 60 * 1000);
          const {
            data: [result]
          } = await pablo
            .addLiquidity(api, liquidityProviderWallet1, picaLBPPoolId, baseAmount, quoteAmount, minMintAmount, true)
            .catch(error => {
              return { data: [error.message] };
            });
          expect(result).to.contain("pablo.MustBeOwner");
        });

        it("Pool owner can add liquidity to the pool", async function () {
          this.timeout(2 * 60 * 1000);
          const {
            data: [result]
          } = await pablo.addLiquidity(
            api,
            composableManagerWallet,
            picaLBPPoolId,
            baseAmount,
            quoteAmount,
            minMintAmount,
            true
          );
        });
      });
    });

    describe("Test 2B trading", function () {
      describe("Test 2B buy", function () {
        it("Users can buy from pool", async function () {
          this.timeout(5 * 60 * 1000);
          await Phase2.waitForLBPPoolStarted(api, picaLBPPoolId);
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
            const picaBalanceBefore = await api.rpc.assets.balanceOf(
              picaAssetId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const usdcBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAssetId.toString(),
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
            await Phase2.verifyPoolLiquidityAdded(
              api,
              picaAssetId,
              usdcAssetId,
              picaUsdcLpTokenId,
              liquidityProviderWallet1.publicKey,
              baseAmount,
              new BN(picaBalanceBefore.toString()),
              new BN(usdcBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });

          it("Pool owner can add liquidity to the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              picaUsdcLpTokenId.toString(),
              composableManagerWallet.publicKey
            );
            const picaBalanceBefore = await api.rpc.assets.balanceOf(
              picaAssetId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const usdcBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAssetId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const {
              data: [result]
            } = await pablo.addLiquidity(
              api,
              composableManagerWallet,
              picaUsdcPoolId,
              baseAmount,
              quoteAmount,
              minMintAmount,
              true
            );
            await Phase2.verifyPoolLiquidityAdded(
              api,
              picaAssetId,
              usdcAssetId,
              ksmUsdcLpTokenId,
              liquidityProviderWallet1.publicKey,
              baseAmount,
              new BN(picaBalanceBefore.toString()),
              new BN(usdcBalanceBefore.toString()),
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
            const picaBalanceBefore = await api.rpc.assets.balanceOf(
              picaAssetId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const usdcBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAssetId.toString(),
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
            await Phase2.verifyPoolLiquidityRemoved(
              api,
              picaAssetId,
              usdcAssetId,
              picaUsdcLpTokenId,
              liquidityProviderWallet1.publicKey,
              baseAmount,
              new BN(picaBalanceBefore.toString()),
              new BN(usdcBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });
          it("Pool owner can remove liquidity from the pool", async function () {
            // ToDo: Update when root can create pools!
            this.skip();
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              picaUsdcLpTokenId.toString(),
              composableManagerWallet.publicKey
            );
            const picaBalanceBefore = await api.rpc.assets.balanceOf(
              picaAssetId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const usdcBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAssetId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
            const baseAmount = 0;
            const quoteAmount = 0;
            const {
              data: [result]
            } = await pablo.sudo.sudoRemoveLiquidity(
              api,
              composableManagerWallet,
              picaUsdcPoolId,
              lpAmount,
              baseAmount,
              quoteAmount
            );
            await Phase2.verifyPoolLiquidityRemoved(
              api,
              picaAssetId,
              usdcAssetId,
              picaUsdcLpTokenId,
              composableManagerWallet.publicKey,
              baseAmount,
              new BN(picaBalanceBefore.toString()),
              new BN(usdcBalanceBefore.toString()),
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
            const amount = 500_000_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.buyTokens(api, traderWallet1, picaUsdcPoolId, assetIdToBuy, amount, minReceive, keepAlive);
          });
        });

        describe("Test 2C:1 sell", function () {
          it("Users can sell to pool", async function () {
            this.timeout(2 * 60 * 1000);
            const assetIdToSell = picaAssetId;
            const amount = 100_000_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.sellTokens(
              api,
              traderWallet1,
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
            } = await pablo.swapTokens(api, traderWallet1, picaUsdcPoolId, pair, amount, minReceive, keepAlive);
          });
        });
      });
    });

    describe("2C:2 PICA/KSM Uniswap Pool", function () {
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
            const picaBalanceBefore = await api.rpc.assets.balanceOf(
              picaAssetId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const ksmBalanceBefore = await api.rpc.assets.balanceOf(
              ksmAssetId.toString(),
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
            await Phase2.verifyPoolLiquidityAdded(
              api,
              picaAssetId,
              ksmAssetId,
              ksmUsdcLpTokenId,
              liquidityProviderWallet1.publicKey,
              baseAmount,
              new BN(picaBalanceBefore.toString()),
              new BN(ksmBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });

          it("Pool owner can add liquidity to the pool", async function () {
            // ToDo: Update when root can create pools!
            this.skip();
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              picaKsmLpTokenId.toString(),
              composableManagerWallet.publicKey
            );
            const picaBalanceBefore = await api.rpc.assets.balanceOf(
              picaAssetId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const ksmBalanceBefore = await api.rpc.assets.balanceOf(
              ksmAssetId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const {
              data: [result]
            } = await pablo.addLiquidity(
              api,
              composableManagerWallet,
              picaKsmPoolId,
              baseAmount,
              quoteAmount,
              minMintAmount,
              true
            );
            await Phase2.verifyPoolLiquidityAdded(
              api,
              ksmAssetId,
              usdcAssetId,
              picaKsmLpTokenId,
              liquidityProviderWallet1.publicKey,
              baseAmount,
              new BN(picaBalanceBefore.toString()),
              new BN(ksmBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });
        });

        describe("Test 2C:2 pool remove liquidity", function () {
          it("Users can remove liquidity from the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              picaKsmLpTokenId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const picaBalanceBefore = await api.rpc.assets.balanceOf(
              picaAssetId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const ksmBalanceBefore = await api.rpc.assets.balanceOf(
              ksmAssetId.toString(),
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
            await Phase2.verifyPoolLiquidityRemoved(
              api,
              picaAssetId,
              ksmAssetId,
              picaKsmLpTokenId,
              liquidityProviderWallet1.publicKey,
              baseAmount,
              new BN(picaBalanceBefore.toString()),
              new BN(ksmBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });
          it("Pool owner can remove liquidity from the pool", async function () {
            this.skip();
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              picaKsmLpTokenId.toString(),
              composableManagerWallet.publicKey
            );
            const picaBalanceBefore = await api.rpc.assets.balanceOf(
              picaAssetId.toString(),
              composableManagerWallet.publicKey
            );
            const ksmBalanceBefore = await api.rpc.assets.balanceOf(
              ksmAssetId.toString(),
              composableManagerWallet.publicKey
            );
            const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
            const baseAmount = 0;
            const quoteAmount = 0;
            const {
              data: [result]
            } = await pablo.removeLiquidity(
              api,
              composableManagerWallet,
              picaKsmPoolId,
              lpAmount,
              baseAmount,
              quoteAmount
            );
            await Phase2.verifyPoolLiquidityRemoved(
              api,
              picaAssetId,
              ksmAssetId,
              picaKsmLpTokenId,
              composableManagerWallet.publicKey,
              baseAmount,
              new BN(picaBalanceBefore.toString()),
              new BN(ksmBalanceBefore.toString()),
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
            } = await pablo.buyTokens(api, traderWallet1, picaKsmPoolId, assetIdToBuy, amount, minReceive, keepAlive);
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
            } = await pablo.sellTokens(api, traderWallet1, picaKsmPoolId, assetIdToSell, amount, minReceive, keepAlive);
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
            } = await pablo.swapTokens(api, traderWallet1, picaKsmPoolId, pair, amount, minReceive, keepAlive);
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
            stableSwap: {
              owner: composableManagerWallet.publicKey,
              pair: {
                base: baseAsset,
                quote: quoteAsset
              },
              amplificationCoefficient: amplificationCoefficient,
              lpToken: 100_000_000_000n,
              feeConfig: {
                feeRate: 100000,
                ownerFeeRate: 200000,
                protocolFeeRate: 1000000
              }
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
              liquidityProviderWallet2.publicKey
            );
            const usdcBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAssetId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const ausdBalanceBefore = await api.rpc.assets.balanceOf(
              ausdAssetId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const {
              data: [result]
            } = await pablo.addLiquidity(
              api,
              liquidityProviderWallet2,
              usdcAusdPoolId,
              baseAmount,
              quoteAmount,
              minMintAmount,
              true
            );
            await Phase2.verifyPoolLiquidityAdded(
              api,
              usdcAssetId,
              ausdAssetId,
              usdcAusdLpTokenId,
              liquidityProviderWallet2.publicKey,
              baseAmount,
              new BN(usdcBalanceBefore.toString()),
              new BN(ausdBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });

          it("Pool owner can add liquidity to the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAusdLpTokenId.toString(),
              composableManagerWallet.publicKey
            );
            const usdcBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAssetId.toString(),
              composableManagerWallet.publicKey
            );
            const ausdBalanceBefore = await api.rpc.assets.balanceOf(
              ausdAssetId.toString(),
              composableManagerWallet.publicKey
            );
            const {
              data: [result]
            } = await pablo.addLiquidity(
              // ToDo: Switch to sudo!
              api,
              composableManagerWallet,
              usdcAusdPoolId,
              baseAmount,
              quoteAmount,
              minMintAmount,
              true
            );
            await Phase2.verifyPoolLiquidityAdded(
              api,
              usdcAssetId,
              ausdAssetId,
              usdcAusdLpTokenId,
              composableManagerWallet.publicKey,
              baseAmount,
              new BN(usdcBalanceBefore.toString()),
              new BN(ausdBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });
        });

        describe("Test 2D:1 pool remove liquidity", function () {
          it("Users can remove liquidity from the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAusdLpTokenId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const usdcBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAssetId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const ausdBalanceBefore = await api.rpc.assets.balanceOf(
              ausdAssetId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
            const baseAmount = 0;
            const quoteAmount = 0;
            const {
              data: [result]
            } = await pablo.removeLiquidity(
              api,
              liquidityProviderWallet2,
              usdcAusdPoolId,
              lpAmount,
              baseAmount,
              quoteAmount
            );
            await Phase2.verifyPoolLiquidityRemoved(
              api,
              usdcAssetId,
              ausdAssetId,
              usdcAusdLpTokenId,
              liquidityProviderWallet2.publicKey,
              baseAmount,
              new BN(usdcBalanceBefore.toString()),
              new BN(ausdBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });
          it("Pool owner (sudo) can remove liquidity from the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAusdLpTokenId.toString(),
              composableManagerWallet.publicKey
            );
            const usdcBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAssetId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const ausdBalanceBefore = await api.rpc.assets.balanceOf(
              ausdAssetId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
            const baseAmount = 0;
            const quoteAmount = 0;
            const {
              data: [result]
            } = await pablo.removeLiquidity(
              // ToDo: Switch to sudo!
              api,
              composableManagerWallet,
              usdcAusdPoolId,
              lpAmount,
              baseAmount,
              quoteAmount
            );
            await Phase2.verifyPoolLiquidityRemoved(
              api,
              usdcAssetId,
              ausdAssetId,
              usdcAusdLpTokenId,
              composableManagerWallet.publicKey,
              baseAmount,
              new BN(usdcBalanceBefore.toString()),
              new BN(ausdBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });
        });
      });

      describe("Test 2D:1 trading", function () {
        describe("Test 2D:! buy", function () {
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

        describe("Test 2D:1 sell", function () {
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

        describe("Test 2D:1 swap", function () {
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

    describe("2D:2 wETH/KSM Uniswap Pool", function () {
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
        wethKsmPoolId = poolId;
        wethKsmLpTokenId = lpTokenId;
      });

      describe("Test 2D:2 pool liquidity", function () {
        describe("Test 2D:2 pool add liquidity", function () {
          it("Users can add liquidity to the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              wethKsmLpTokenId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const wethBalanceBefore = await api.rpc.assets.balanceOf(
              wethAssetId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const ksmBalanceBefore = await api.rpc.assets.balanceOf(
              ksmAssetId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const {
              data: [result]
            } = await pablo.addLiquidity(
              api,
              liquidityProviderWallet2,
              wethKsmPoolId,
              baseAmount,
              quoteAmount,
              minMintAmount,
              true
            );
            await Phase2.verifyPoolLiquidityAdded(
              api,
              wethAssetId,
              ksmAssetId,
              wethKsmLpTokenId,
              liquidityProviderWallet2.publicKey,
              baseAmount,
              new BN(wethBalanceBefore.toString()),
              new BN(ksmBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });

          it("Pool owner can add liquidity to the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              wethKsmLpTokenId.toString(),
              composableManagerWallet.publicKey
            );
            const wethBalanceBefore = await api.rpc.assets.balanceOf(
              wethAssetId.toString(),
              composableManagerWallet.publicKey
            );
            const ksmBalanceBefore = await api.rpc.assets.balanceOf(
              ksmAssetId.toString(),
              composableManagerWallet.publicKey
            );
            const {
              data: [result]
            } = await pablo.addLiquidity(
              // ToDo: Switch to sudo!
              api,
              composableManagerWallet,
              wethKsmPoolId,
              baseAmount,
              quoteAmount,
              minMintAmount,
              true
            );
            await Phase2.verifyPoolLiquidityAdded(
              api,
              ksmAssetId,
              usdcAssetId,
              ksmUsdcLpTokenId,
              composableManagerWallet.publicKey,
              baseAmount,
              new BN(wethBalanceBefore.toString()),
              new BN(ksmBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });
        });

        describe("Test 2C:2 pool remove liquidity", function () {
          it("Users can remove liquidity from the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              wethKsmLpTokenId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const wethBalanceBefore = await api.rpc.assets.balanceOf(
              wethAssetId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const ksmBalanceBefore = await api.rpc.assets.balanceOf(
              ksmAssetId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
            const baseAmount = 0;
            const quoteAmount = 0;
            const {
              data: [result]
            } = await pablo.removeLiquidity(
              api,
              liquidityProviderWallet2,
              wethKsmPoolId,
              lpAmount,
              baseAmount,
              quoteAmount
            );
            await Phase2.verifyPoolLiquidityRemoved(
              api,
              wethAssetId,
              ksmAssetId,
              wethKsmLpTokenId,
              liquidityProviderWallet2.publicKey,
              baseAmount,
              new BN(wethBalanceBefore.toString()),
              new BN(ksmBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });
          it("Pool owner can remove liquidity from the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              wethKsmPoolId.toString(),
              composableManagerWallet.publicKey
            );
            const wethBalanceBefore = await api.rpc.assets.balanceOf(
              wethAssetId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const ksmBalanceBefore = await api.rpc.assets.balanceOf(
              ksmAssetId.toString(),
              liquidityProviderWallet1.publicKey
            );
            const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
            const baseAmount = 0;
            const quoteAmount = 0;
            const {
              data: [result]
            } = await pablo.removeLiquidity(
              // ToDo: Switch to sudo!
              api,
              composableManagerWallet,
              wethKsmPoolId,
              lpAmount,
              baseAmount,
              quoteAmount
            );
            await Phase2.verifyPoolLiquidityRemoved(
              api,
              wethAssetId,
              ksmAssetId,
              wethKsmLpTokenId,
              liquidityProviderWallet2.publicKey,
              baseAmount,
              new BN(wethBalanceBefore.toString()),
              new BN(ksmBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });
        });
      });

      describe("Test 2D:2 trading", function () {
        describe("Test 2D:2 buy", function () {
          it("Users can buy from pool", async function () {
            this.timeout(2 * 60 * 1000);
            const assetIdToBuy = wethAssetId;
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.buyTokens(api, traderWallet1, wethKsmPoolId, assetIdToBuy, amount, minReceive, keepAlive);
          });
        });

        describe("Test 2D:2 sell", function () {
          it("Users can sell to pool", async function () {
            this.timeout(2 * 60 * 1000);
            const assetIdToSell = wethAssetId;
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.sellTokens(api, traderWallet1, wethKsmPoolId, assetIdToSell, amount, minReceive, keepAlive);
          });
        });

        describe("Test 2D:2 swap", function () {
          it("Users can swap in the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const pair = { base: wethAssetId, quote: ksmAssetId };
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.swapTokens(api, traderWallet1, wethKsmPoolId, pair, amount, minReceive, keepAlive);
          });
        });
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
        wbtcKsmPoolId = poolId;
        wbtcKsmLpTokenId = lpTokenId;
      });
      describe("Test 2D:3 pool liquidity", function () {
        describe("Test 2D:3 pool add liquidity", function () {
          it("Users can add liquidity to the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              wbtcKsmLpTokenId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const wbtcBalanceBefore = await api.rpc.assets.balanceOf(
              btcAssetId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const ksmBalanceBefore = await api.rpc.assets.balanceOf(
              ksmAssetId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const {
              data: [result]
            } = await pablo.addLiquidity(
              api,
              liquidityProviderWallet2,
              wbtcKsmPoolId,
              baseAmount,
              quoteAmount,
              minMintAmount,
              true
            );
            await Phase2.verifyPoolLiquidityAdded(
              api,
              btcAssetId,
              ksmAssetId,
              wbtcKsmLpTokenId,
              liquidityProviderWallet2.publicKey,
              baseAmount,
              new BN(wbtcBalanceBefore.toString()),
              new BN(ksmBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });

          it("Pool owner can add liquidity to the pool", async function () {
            this.skip();
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              wbtcKsmLpTokenId.toString(),
              composableManagerWallet.publicKey
            );
            const wbtcBalanceBefore = await api.rpc.assets.balanceOf(
              btcAssetId.toString(),
              composableManagerWallet.publicKey
            );
            const ksmBalanceBefore = await api.rpc.assets.balanceOf(
              ksmAssetId.toString(),
              composableManagerWallet.publicKey
            );
            const {
              data: [result]
            } = await pablo.addLiquidity(
              // ToDo: Switch to sudo!
              api,
              composableManagerWallet,
              wbtcKsmPoolId,
              baseAmount,
              quoteAmount,
              minMintAmount,
              true
            );
            await Phase2.verifyPoolLiquidityAdded(
              api,
              btcAssetId,
              ksmAssetId,
              ksmUsdcLpTokenId,
              composableManagerWallet.publicKey,
              baseAmount,
              new BN(wbtcBalanceBefore.toString()),
              new BN(ksmBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });
        });

        describe("Test 2D:3 pool remove liquidity", function () {
          it("Users can remove liquidity from the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              wbtcKsmLpTokenId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const wbtcBalanceBefore = await api.rpc.assets.balanceOf(
              btcAssetId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const ksmBalanceBefore = await api.rpc.assets.balanceOf(
              ksmAssetId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
            const baseAmount = 0;
            const quoteAmount = 0;
            const {
              data: [result]
            } = await pablo.removeLiquidity(
              api,
              liquidityProviderWallet2,
              wbtcKsmPoolId,
              lpAmount,
              baseAmount,
              quoteAmount
            );
            await Phase2.verifyPoolLiquidityRemoved(
              api,
              wethAssetId,
              ksmAssetId,
              wethKsmLpTokenId,
              liquidityProviderWallet2.publicKey,
              baseAmount,
              new BN(wbtcBalanceBefore.toString()),
              new BN(ksmBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });
          it("Pool owner can remove liquidity from the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              wbtcKsmLpTokenId.toString(),
              composableManagerWallet.publicKey
            );
            const wbtcBalanceBefore = await api.rpc.assets.balanceOf(
              btcAssetId.toString(),
              composableManagerWallet.publicKey
            );
            const ksmBalanceBefore = await api.rpc.assets.balanceOf(
              ksmAssetId.toString(),
              composableManagerWallet.publicKey
            );
            const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
            const baseAmount = 0;
            const quoteAmount = 0;
            const {
              data: [result]
            } = await pablo.removeLiquidity(
              // ToDo: Switch to sudo!
              api,
              composableManagerWallet,
              wbtcKsmPoolId,
              lpAmount,
              baseAmount,
              quoteAmount
            );
            await Phase2.verifyPoolLiquidityRemoved(
              api,
              wethAssetId,
              ksmAssetId,
              wethKsmLpTokenId,
              composableManagerWallet.publicKey,
              baseAmount,
              new BN(wbtcBalanceBefore.toString()),
              new BN(ksmBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });
        });
      });

      describe("Test 2D:3 trading", function () {
        describe("Test 2D:3 buy", function () {
          it("Users can buy from pool", async function () {
            this.timeout(2 * 60 * 1000);
            const assetIdToBuy = btcAssetId;
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.buyTokens(api, traderWallet1, wbtcKsmPoolId, assetIdToBuy, amount, minReceive, keepAlive);
          });
        });

        describe("Test 2D:3 sell", function () {
          it("Users can sell to pool", async function () {
            this.timeout(2 * 60 * 1000);
            const assetIdToSell = btcAssetId;
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.sellTokens(api, traderWallet1, wbtcKsmPoolId, assetIdToSell, amount, minReceive, keepAlive);
          });
        });

        describe("Test 2D:3 swap", function () {
          it("Users can swap in the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const pair = { base: btcAssetId, quote: ksmAssetId };
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.swapTokens(api, traderWallet1, wbtcKsmPoolId, pair, amount, minReceive, keepAlive);
          });
        });
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
            stableSwap: {
              owner: composableManagerWallet.publicKey,
              pair: {
                base: baseAsset,
                quote: quoteAsset
              },
              amplificationCoefficient: amplificationCoefficient,
              lpToken: 100_000_000_000n,
              feeConfig: {
                feeRate: 100000,
                ownerFeeRate: 200000,
                protocolFeeRate: 1000000
              }
            }
          })
        );
        usdcUsdtPoolId = poolId;
        usdcUsdtLpTokenId = lpTokenId;
      });

      describe("Test 2D:4 pool liquidity", function () {
        describe("Test 2D:4 pool add liquidity", function () {
          it("Users can add liquidity to the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              usdcUsdtLpTokenId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const usdcBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAssetId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const usdtBalanceBefore = await api.rpc.assets.balanceOf(
              usdtAssetId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const {
              data: [result]
            } = await pablo.addLiquidity(
              api,
              liquidityProviderWallet2,
              usdcUsdtPoolId,
              baseAmount,
              quoteAmount,
              minMintAmount,
              true
            );
            await Phase2.verifyPoolLiquidityAdded(
              api,
              usdcAssetId,
              usdtAssetId,
              usdcUsdtLpTokenId,
              liquidityProviderWallet2.publicKey,
              baseAmount,
              new BN(usdcBalanceBefore.toString()),
              new BN(usdtBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });

          it("Pool owner can add liquidity to the pool", async function () {
            // ToDo: Update when root can create pools!
            this.skip();
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              usdcUsdtLpTokenId.toString(),
              composableManagerWallet.publicKey
            );
            const usdcBalanceBefore = await api.rpc.assets.balanceOf(
              usdcAssetId.toString(),
              composableManagerWallet.publicKey
            );
            const usdtBalanceBefore = await api.rpc.assets.balanceOf(
              usdtAssetId.toString(),
              composableManagerWallet.publicKey
            );
            const {
              data: [result]
            } = await pablo.addLiquidity(
              // ToDo: Switch to sudo!
              api,
              composableManagerWallet,
              usdcUsdtPoolId,
              baseAmount,
              quoteAmount,
              minMintAmount,
              true
            );
            await Phase2.verifyPoolLiquidityAdded(
              api,
              usdcAssetId,
              usdtAssetId,
              ksmUsdcLpTokenId,
              composableManagerWallet.publicKey,
              baseAmount,
              new BN(usdcBalanceBefore.toString()),
              new BN(usdtBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });
        });

        describe("Test 2D:4 pool remove liquidity", function () {
          it("Users can remove liquidity from the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              usdcUsdtLpTokenId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const usdcBalanceBefore = await api.rpc.assets.balanceOf(
              usdcUsdtLpTokenId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const usdtBalanceBefore = await api.rpc.assets.balanceOf(
              usdcUsdtLpTokenId.toString(),
              liquidityProviderWallet2.publicKey
            );
            const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
            const baseAmount = 0;
            const quoteAmount = 0;
            const {
              data: [result]
            } = await pablo.removeLiquidity(
              api,
              liquidityProviderWallet2,
              usdcUsdtPoolId,
              lpAmount,
              baseAmount,
              quoteAmount
            );
            await Phase2.verifyPoolLiquidityRemoved(
              api,
              usdcAssetId,
              usdtAssetId,
              usdcUsdtLpTokenId,
              liquidityProviderWallet2.publicKey,
              baseAmount,
              new BN(usdcBalanceBefore.toString()),
              new BN(usdtBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });
          it("Pool owner can remove liquidity from the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const lpTokenBalanceBefore = await api.rpc.assets.balanceOf(
              usdcUsdtLpTokenId.toString(),
              composableManagerWallet.publicKey
            );
            const usdcBalanceBefore = await api.rpc.assets.balanceOf(
              usdcUsdtLpTokenId.toString(),
              composableManagerWallet.publicKey
            );
            const usdtBalanceBefore = await api.rpc.assets.balanceOf(
              usdcUsdtLpTokenId.toString(),
              composableManagerWallet.publicKey
            );
            const lpAmount = new BN(lpTokenBalanceBefore.toString()).div(new BN(2));
            const baseAmount = 0;
            const quoteAmount = 0;
            const {
              data: [result]
            } = await pablo.removeLiquidity(
              // ToDo: Switch to sudo!
              api,
              composableManagerWallet,
              usdcUsdtPoolId,
              lpAmount,
              baseAmount,
              quoteAmount
            );
            await Phase2.verifyPoolLiquidityRemoved(
              api,
              usdcAssetId,
              usdtAssetId,
              usdcUsdtLpTokenId,
              composableManagerWallet.publicKey,
              baseAmount,
              new BN(usdcBalanceBefore.toString()),
              new BN(usdtBalanceBefore.toString()),
              new BN(lpTokenBalanceBefore.toString())
            );
          });
        });
      });

      describe("Test 2D:3 trading", function () {
        describe("Test 2D:3 buy", function () {
          it("Users can buy from pool", async function () {
            this.timeout(2 * 60 * 1000);
            const assetIdToBuy = usdcAssetId;
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.buyTokens(api, traderWallet1, usdcUsdtPoolId, assetIdToBuy, amount, minReceive, keepAlive);
          });
        });

        describe("Test 2D:3 sell", function () {
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
              traderWallet1,
              usdcUsdtPoolId,
              assetIdToSell,
              amount,
              minReceive,
              keepAlive
            );
          });
        });

        describe("Test 2D:3 swap", function () {
          it("Users can swap in the pool", async function () {
            this.timeout(2 * 60 * 1000);
            const pair = { base: usdcAssetId, quote: usdtAssetId };
            const amount = 100_000_000_000n;
            const minReceive = 0;
            const keepAlive = true;
            const {
              data: [result]
            } = await pablo.swapTokens(api, traderWallet1, usdcUsdtPoolId, pair, amount, minReceive, keepAlive);
          });
        });
      });
    });
  });
});
