import pabloTestConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { mintAssetsToWallet, Pica } from "@composable/utils/mintingHelper";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { ApiPromise } from "@polkadot/api";
import { sendAndWaitForSuccess, waitForBlocks } from "@composable/utils/polkadotjs";
import { expect } from "chai";
import BN from "bn.js";
import { OrmlTokensAccountData } from "@composable/types/interfaces";
import _ from "lodash";
import {
  calculateInGivenOut,
  calculateOutGivenIn
} from "@composabletests/tests/pablo/testHandlers/constantProduct/weightedMath";
import BigNumber from "bignumber.js";

/**
 * Pablo Constant Product Integration Test Suite
 *
 * We're not creating pools due to the current state of Pablo!
 * And since the current pools are created during a chain migration, they're hardcoded for now.
 */

const hardCodedPool1 = {
  poolId: 0,
  baseAssetId: 4,
  quoteAssetId: 130,
  lpTokenId: 105,
  poolWalletAddress: "5w3oyasYQg6xWPRnTBT5A2zLnRDBngFZQP2ify51JjUfKCDD"
};

const hardCodedPool2 = {
  poolId: 1,
  baseAssetId: 1,
  quoteAssetId: 130,
  lpTokenId: 106,
  poolWalletAddress: "5w3oyasYQg6xWPRnTBTLu4XvtutPFEMS93yEDukqmZMPaznS"
};

const DEFAULT_FEE = 3000; // 0.3 Percent

const DEFAULT_LIQUIDITY_AMOUNT_TO_ADD = Pica(10_000);

describe("tx.constantProductDex Tests", function () {
  if (!pabloTestConfiguration.enabledTests.enabled) {
    console.log("Constant Product Tests are being skipped...");
    return;
  }
  this.timeout(3 * 60 * 1000);
  let api: ApiPromise;
  let sudoKey: KeyringPair,
    poolOwnerWallet: KeyringPair,
    walletLpProvider1: KeyringPair,
    walletLpProvider2: KeyringPair,
    walletTrader1: KeyringPair;
  let poolId1: number, poolId2: number, poolId3: number;

  before("Initialize variables", async function () {
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice, devWalletEve, devWalletFerdie } = getDevWallets(newKeyring);

    sudoKey = devWalletAlice;
    poolOwnerWallet = devWalletFerdie.derive("/test/pablo/pool/owner");
    walletLpProvider1 = devWalletFerdie.derive("/test/pablo/lp/provider/1");
    walletLpProvider2 = devWalletFerdie.derive("/test/pablo/lp/provider/2");
    walletTrader1 = devWalletFerdie.derive("/test/pablo/trader/1");
  });

  before("Minting assets", async function () {
    // await mintAssetsToWallet(api, poolOwnerWallet, sudoKey, [1]);
    await mintAssetsToWallet(
      api,
      walletLpProvider1,
      sudoKey,
      [1, 4, 130],
      10000000000000n * DEFAULT_LIQUIDITY_AMOUNT_TO_ADD
    );
    await mintAssetsToWallet(api, walletLpProvider2, sudoKey, [1]);
    await mintAssetsToWallet(api, walletTrader1, sudoKey, [1, 4, 130]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  describe.skip("1. Pool creation", function () {
    it("#1.1  I can, as sudo, create a new Pablo Constant Product pool.", async function () {
      const owner = api.createType("AccountId32", poolOwnerWallet.publicKey);
      const poolConfiguration = api.createType("PalletPabloPoolInitConfiguration", {
        DualAssetConstantProduct: {
          owner: owner,
          assetsWeights: api.createType("BTreeMap<u128, Permill>", {
            "1": 500_000,
            "131": 500_000
          }),
          fee: api.createType("Permill", 10_000)
        }
      });

      const {
        data: [resultPoolId, resultOwner, resultAssets]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.pablo.PoolCreated.is,
        api.tx.sudo.sudo(api.tx.pablo.create(poolConfiguration))
      );
      poolId1 = resultPoolId.toNumber();
      expect(resultOwner.toString()).to.be.equal(owner.toString());
      console.debug(resultAssets.toHuman());
    });

    it("#1.2  I can, as sudo, create a new Pablo Constant Product pool, for assets which already belong to an existing pool.", async function () {
      const owner = api.createType("AccountId32", poolOwnerWallet.publicKey);
      const poolConfiguration = api.createType("PalletPabloPoolInitConfiguration", {
        DualAssetConstantProduct: {
          owner: owner,
          assetsWeights: api.createType("BTreeMap<u128, Permill>", {
            "4": 500_000,
            "131": 500_000
          }),
          fee: api.createType("Permill", 10_000)
        }
      });

      const {
        data: [resultPoolId, resultOwner, resultAssets]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.pablo.PoolCreated.is,
        api.tx.sudo.sudo(api.tx.pablo.create(poolConfiguration))
      );
      poolId2 = resultPoolId.toNumber();
      expect(resultOwner.toString()).to.be.equal(owner.toString());
      console.debug(resultAssets.toHuman());
    });

    it("#1.3  User wallets can not create new Pablo Constant Product pools.", async function () {
      const owner = api.createType("AccountId32", poolOwnerWallet.publicKey);
      const poolConfiguration = api.createType("PalletPabloPoolInitConfiguration", {
        DualAssetConstantProduct: {
          owner: owner,
          assetsWeights: api.createType("BTreeMap<u128, Permill>", {
            "1": 500_000,
            "131": 500_000
          }),
          fee: api.createType("Permill", 10_000)
        }
      });

      const res = await sendAndWaitForSuccess(
        api,
        poolOwnerWallet,
        api.events.pablo.PoolCreated.is,
        api.tx.pablo.create(poolConfiguration)
      ).catch(function (exc) {
        return exc;
      });
      expect(res.toString()).to.contain("BadOrigin");
    });

    it("#1.4  I can, as sudo, create a new Pablo Constant Product pool with 0 fees.", async function () {
      const owner = api.createType("AccountId32", poolOwnerWallet.publicKey);
      const poolConfiguration = api.createType("PalletPabloPoolInitConfiguration", {
        DualAssetConstantProduct: {
          owner: owner,
          assetsWeights: api.createType("BTreeMap<u128, Permill>", {
            "1": 500_000,
            "131": 500_000
          }),
          fee: api.createType("Permill", 0)
        }
      });

      const {
        data: [resultPoolId, resultOwner, resultAssets]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.pablo.PoolCreated.is,
        api.tx.sudo.sudo(api.tx.pablo.create(poolConfiguration))
      );
      poolId3 = resultPoolId.toNumber();
      expect(resultOwner.toString()).to.be.equal(owner.toString());
      console.debug(resultAssets.toHuman());
    });

    it("#1.5  I can not, as sudo, create a new Pablo Constant Product pool with fees greater than 100%.", async function () {
      const owner = api.createType("AccountId32", poolOwnerWallet.publicKey);
      const poolConfiguration = api.createType("PalletPabloPoolInitConfiguration", {
        DualAssetConstantProduct: {
          owner: owner,
          assetsWeights: api.createType("BTreeMap<u128, Permill>", {
            "1": 500_000,
            "131": 500_000
          }),
          fee: api.createType("Permill", 1_250_000) // 125% fee
        }
      });

      const res = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.pablo.PoolCreated.is,
        api.tx.sudo.sudo(api.tx.pablo.create(poolConfiguration))
      ).catch(function (exc) {
        return exc;
      });

      expect(res.toString()).to.contain("RpcError: 1002");
    });

    it("#1.6  I can not, as sudo, create a new Pablo Constant Product pool with the base asset being equal to the quote asset.", async function () {
      const owner = api.createType("AccountId32", poolOwnerWallet.publicKey);
      const poolConfiguration = api.createType("PalletPabloPoolInitConfiguration", {
        DualAssetConstantProduct: {
          owner: owner,
          assetsWeights: api.createType("BTreeMap<u128, Permill>", {
            "1": 500_000,
            // @ts-ignore
            "1": 500_000
          }),
          fee: api.createType("Permill", 10_000)
        }
      });

      const res = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.pablo.create(poolConfiguration))
      ).catch(function (exc) {
        return exc;
      });
      expect(res.toString()).to.contain('"index":65,"error":"0x0b000000"');
    });
  });

  describe("2. Providing liquidity", function () {
    it("#2.1  I can provide liquidity to the predefined KSM<>USDT pool. ~~newly created pool. #1.1~~", async function () {
      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(10),
        "130": Pica(100)
      });

      const expectedAmountLpTokens = await api.rpc.pablo.simulateAddLiquidity(
        walletLpProvider1.address,
        hardCodedPool1.poolId.toString(), // ToDo: Update pool id w/ created pool!
        api.createType("BTreeMap<u128, u128>", assets)
      );
      const baseAssetFundsCurrentlyInPoolsBefore = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.baseAssetId
      );
      const quoteAssetFundsCurrentlyInPoolsBefore = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.quoteAssetId
      );

      console.debug("KSM in pool:", baseAssetFundsCurrentlyInPoolsBefore.free.toString());
      console.debug("USDT in pool:", quoteAssetFundsCurrentlyInPoolsBefore.free.toString());
      const {
        data: [resultWho, resultPoolId, resultAssetsAmount, resultMintedLp]
      } = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityAdded.is,
        // Parameters: poolId, assetsMap, minMintAmount, keepAlive
        api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assets, 0, true) // ToDo: Update pool id w/ created pool!
      );
      // ToDo

      const baseAssetFundsCurrentlyInPoolsAfter = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.baseAssetId
      );
      const quoteAssetFundsCurrentlyInPoolsAfter = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.quoteAssetId
      );

      console.debug("KSM in pool:", baseAssetFundsCurrentlyInPoolsAfter.free.toString());
      console.debug("USDT in pool:", quoteAssetFundsCurrentlyInPoolsAfter.free.toString());

      expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletLpProvider1.publicKey).toString());
      expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPool1.poolId)); // ToDo: Update pool id w/ created pool!
      expect(resultAssetsAmount).to.be.eql(assets);
      // expect(new BN(resultMintedLp.toString())).to.be.bignumber.closeTo(new BN(expectedAmountLpTokens.toString()), 1000)
      // console.debug("MintedLp", resultMintedLp);
      // console.debug("ExpectedLp", expectedAmountLpTokens);
    });

    it("#2.2  I can transfer my LP tokens to another user.", async function () {
      const poolQuery = await api.query.pablo.pools(0);
      const lpTokenId = poolQuery.unwrap().asDualAssetConstantProduct.lpToken;
      const receivingWallet = walletLpProvider2.publicKey;
      const lpAmount = <OrmlTokensAccountData>await api.query.tokens.accounts(walletLpProvider1.publicKey, lpTokenId);
      const amountToTransfer = lpAmount.free.muln(0.5);
      const {
        data: [resultCurrencyId, resultFrom, resultTo, resultAmount]
      } = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.tokens.Transfer.is,
        api.tx.assets.transfer(lpTokenId, receivingWallet, amountToTransfer, false)
      );
      expect(resultCurrencyId).to.be.bignumber.equal(lpTokenId);
      expect(resultFrom.toString()).to.be.equal(api.createType("AccountId32", walletLpProvider1.publicKey).toString());
      expect(resultTo.toString()).to.be.equal(api.createType("AccountId32", walletLpProvider2.publicKey).toString());
      expect(resultAmount).to.be.bignumber.equal(amountToTransfer);
    });

    it.skip("#2.3  I can not add only the base or quote asset as liquidity", async function () {
      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(1_000_000000)
      });

      const exc = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityAdded.is,
        api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assets, 0, true) // ToDo: Update pool id.
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("pablo.IncorrectAssetAmounts:");
    });

    it("#2.4  I can not add liquidity amounts of 0.", async function () {
      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(0),
        "130": Pica(0)
      });

      const exc = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityAdded.is,
        api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assets, 0, true) // ToDo: Update pool id.
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("pablo.InvalidAmount");
    });

    it("#2.5  I can not add liquidity without respecting the pools ratio.", async function () {
      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(1),
        "130": Pica(99999999)
      });

      const exc = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityAdded.is,
        api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assets, 0, true) // ToDo: Update pool id.
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("pablo.IncorrectAssetAmounts:");

      const assets2 = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(50),
        "130": Pica(50)
      });

      const exc2 = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityAdded.is,
        api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assets, 0, true) // ToDo: Update pool id.
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("pablo.IncorrectAssetAmounts:");

      const exc3 = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityAdded.is,
        api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assets, 0, true) // ToDo: Update pool id.
      ).catch(exc => exc);
      expect(exc3.toString()).to.contain("pablo.IncorrectAssetAmounts:");

      const assets4 = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(33),
        "130": Pica(67)
      });

      const exc4 = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityAdded.is,
        api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assets, 0, true) // ToDo: Update pool id.
      ).catch(exc => exc);
      expect(exc4.toString()).to.contain("pablo.IncorrectAssetAmounts:");
    });
  });

  describe("4. Trading pt. 1", function () {
    it.skip("#4.9  I can not buy in a pool without liquidity.", async function () {
      const exc = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.buy(hardCodedPool2.poolId, 131, { assetId: 1_000, amount: 10_000 }, false) // ToDo: Update poolId & amounts if necessary!
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("pablo.AssetNotFound");
    });
  });

  describe("2. Providing liquidity pt. 2", function () {
    it("#2.6  I can add liquidity with a defined `minMintAmount`.", async function () {
      const assets = api.createType("BTreeMap<u128, u128>", {
        // ToDo: Update
        "1": Pica(10_000),
        "130": Pica(100_000)
      });

      const expectedAmountLpTokens = await api.rpc.pablo.simulateAddLiquidity(
        walletLpProvider1.address,
        hardCodedPool2.poolId.toString(),
        api.createType("BTreeMap<u128, u128>", assets)
      );

      const {
        data: [resultWho, resultPoolId, resultAssetsAmount, resultMintedLp]
      } = await sendAndWaitForSuccess(
        api,
        walletLpProvider1, // ToDo: Update
        api.events.pablo.LiquidityAdded.is,
        api.tx.pablo.addLiquidity(hardCodedPool2.poolId, assets, Pica(100), true)
      );

      expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletLpProvider1.publicKey).toString());
      expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPool2.poolId)); // ToDo: Update pool id w/ created pool!
      expect(resultAssetsAmount).to.be.eql(assets);
      // expect(new BN(resultMintedLp.toString())).to.be.bignumber.closeTo(new BN(expectedAmountLpTokens.toString()), 1000)
      console.debug("MintedLp", resultMintedLp);
      console.debug("ExpectedLp", expectedAmountLpTokens);
    });

    it("#2.7  I can add liquidity to a pool with already available liquidity.", async function () {
      const assets = api.createType("BTreeMap<u128, u128>", {
        // ToDo: Update
        "4": Pica(10_000),
        "130": Pica(100_000)
      });

      const expectedAmountLpTokens = await api.rpc.pablo.simulateAddLiquidity(
        walletLpProvider1.address,
        hardCodedPool1.poolId.toString(),
        api.createType("BTreeMap<u128, u128>", assets)
      );

      const {
        data: [resultWho, resultPoolId, resultAssetsAmount, resultMintedLp]
      } = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityAdded.is,
        api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assets, 0, true)
      );
      // ToDo

      expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletLpProvider1.publicKey).toString());
      expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPool1.poolId)); // ToDo: Update pool id w/ created pool!
      expect(resultAssetsAmount).to.be.eql(assets);
      // expect(new BN(resultMintedLp.toString())).to.be.bignumber.closeTo(new BN(expectedAmountLpTokens.toString()), 1000)
      console.debug("MintedLp", resultMintedLp);
      console.debug("ExpectedLp", expectedAmountLpTokens);
    });
  });

  describe("4. Trading pt. 2", function () {
    it.skip("#4.1  I can not buy an amount more than available liquidity.", async function () {
      const exc = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.buy(hardCodedPool1.poolId, 130, { assetId: 4, amount: Pica(Number.MAX_SAFE_INTEGER * 0.8) }, false)
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("Error: Other");
    });

    it("#4.2  I can not buy an asset which isn't part of the pool.", async function () {
      const exc = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.buy(hardCodedPool1.poolId, 130, { assetId: 1, amount: Pica(1_000) }, false)
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("pablo.AssetNotFound");
    });

    it("#4.3  I can not swap in a pool with assets that aren't listed in that pool.", async function () {
      const exc = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.swap(
          hardCodedPool1.poolId,
          { assetId: 130, amount: Pica(9999999999999999999n) },
          {
            assetId: 1,
            amount: Pica(9999999999999999999n)
          },
          false
        )
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("pablo.AssetNotFound");
    });

    it(
      "#4.4  I can swap an amount, and provided by the amounts i want to give in, " +
        "and it'll be adjusted by the `outGivenIn` formula.",
      async function () {
        /*
        ToDo:
        - Check pool wallet funds
        - Check user funds
        -
         */
        const amount = Pica(1);

        const baseAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.baseAssetId
        );
        const quoteAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.quoteAssetId
        );

        const {
          data: [
            resultPoolId,
            resultWho,
            resultBaseAsset,
            resultQuoteAsset,
            resultBaseAmount,
            resultQuoteAmount,
            resultFee
          ]
        } = await sendAndWaitForSuccess(
          api,
          walletTrader1,
          api.events.pablo.Swapped.is,
          api.tx.pablo.swap(
            hardCodedPool1.poolId,
            { assetId: hardCodedPool1.quoteAssetId, amount: amount },
            {
              assetId: hardCodedPool1.baseAssetId,
              amount: 0
            },
            false
          )
        );
        expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPool1.poolId));
        expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletTrader1.publicKey).toString());
        expect(resultBaseAsset).to.be.bignumber.equal(new BN(hardCodedPool1.baseAssetId));
        expect(resultQuoteAsset).to.be.bignumber.equal(new BN(hardCodedPool1.quoteAssetId));

        console.debug("KSM in pool:", baseAssetFundsCurrentlyInPoolsBeforeTx.free.toString());
        console.debug("USDT in pool:", quoteAssetFundsCurrentlyInPoolsBeforeTx.free.toString());
        const expectedAmountOut = calculateOutGivenIn(
          BigNumber(baseAssetFundsCurrentlyInPoolsBeforeTx.free.toString()),
          BigNumber(quoteAssetFundsCurrentlyInPoolsBeforeTx.free.toString()),
          BigNumber(amount.toString()),
          BigNumber(5),
          BigNumber(5)
        );
        const expectedReducedByFee = expectedAmountOut.minus(BigNumber(resultFee.fee.toString()));
        debugger;
        expect(resultBaseAmount).to.be.bignumber.closeTo(
          new BN(expectedReducedByFee.toFixed(0).toString()),
          300_000_000_000n.toString()
        );
      }
    );

    it(
      "#4.5  I can buy an amount, and provided by the amount i want to get out, " +
        "and it'll be adjusted by the `inGivenOut` formula.",
      async function () {
        const amount = Pica(1);

        const baseAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.baseAssetId
        );
        const quoteAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.quoteAssetId
        );
        const {
          data: [
            resultPoolId,
            resultWho,
            resultBaseAsset,
            resultQuoteAsset,
            resultBaseAmount,
            resultQuoteAmount,
            resultFee
          ]
        } = await sendAndWaitForSuccess(
          api,
          walletTrader1,
          api.events.pablo.Swapped.is,
          api.tx.pablo.buy(hardCodedPool1.poolId, 130, { assetId: 4, amount: amount }, false)
        );
        console.debug("KSM in pool:", baseAssetFundsCurrentlyInPoolsBeforeTx.free.toString());
        console.debug("USDT in pool:", quoteAssetFundsCurrentlyInPoolsBeforeTx.free.toString());

        const expectedAmountIn = calculateInGivenOut(
          BigNumber(baseAssetFundsCurrentlyInPoolsBeforeTx.free.toString()),
          BigNumber(quoteAssetFundsCurrentlyInPoolsBeforeTx.free.toString()),
          BigNumber(amount.toString()),
          BigNumber(5),
          BigNumber(5)
        );
        const expectedReducedByFee = expectedAmountIn.plus(BigNumber(resultFee.fee.toString()));
        debugger;
        expect(resultBaseAmount).to.be.bignumber.closeTo(
          new BN(expectedReducedByFee.toFixed(0).toString()),
          300_000_000_000n.toString()
        );
      }
    );

    it.skip("#4.6  I can not buy 0 amounts of any asset.", async function () {
      const exc = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.buy(hardCodedPool1.poolId, 130, { assetId: 4, amount: 0 }, false)
      ).catch(exc => exc);
      expect(exc.toString()).to.contain('{"arithmetic":"DivisionByZero"}');
    });

    it.skip("#4.19  I can not swap 0 amounts of any asset.", async function () {
      const exc = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.swap(hardCodedPool1.poolId, { assetId: 130, amount: 0 }, { assetId: 4, amount: 0 }, false)
      ).catch(exc => exc);
      expect(exc.toString()).to.contain('{"arithmetic":"DivisionByZero"}');
    });

    it("#4.7  I can not buy all of the available liquidity of a pool.", async function () {
      const baseAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.baseAssetId
      );

      const {
        data: [
          resultPoolId,
          resultWho,
          resultBaseAsset,
          resultQuoteAsset,
          resultBaseAmount,
          resultQuoteAmount,
          resultFee
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.buy(
          hardCodedPool1.poolId,
          130,
          {
            assetId: 4,
            amount: baseAssetFundsCurrentlyInPoolsBeforeTx.free
          },
          false
        )
      );
    });

    it.skip("#4.8  I can not buy with the base asset being the same as the quote asset.", async function () {
      const exc = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.buy(hardCodedPool1.poolId, 4, { assetId: 4, amount: Pica(100) }, false)
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("Error: Other");
      await waitForBlocks(api);
    });

    it("#4.20  I can not swap with the minimum amount requested being the same as the inAsset.", async function () {
      const exc = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.swap(
          hardCodedPool1.poolId,
          { assetId: 4, amount: Pica(100) },
          {
            assetId: 4,
            amount: Pica(100)
          },
          false
        )
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("pablo.CannotRespectMinimumRequested");
    });

    it("#4.10 I can not swap in a pool without liquidity.", async function () {
      const exc = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.swap(hardCodedPool1.poolId, { assetId: 130, amount: 1_000 }, { assetId: 4, amount: 10_000 }, false) // ToDo: Update poolId & amounts if necessary!
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("pablo.CannotRespectMinimumRequested");
    });

    it("#4.11 I can not buy or swap in a pool that doesn't exist.", async function () {
      const err = await Promise.all([
        sendAndWaitForSuccess(
          api,
          walletTrader1,
          api.events.pablo.Swapped.is,
          api.tx.pablo.buy(1337, 131, { assetId: 1, amount: 10_000 }, false) // ToDo: Update poolId & amounts if necessary!
        ),
        sendAndWaitForSuccess(
          api,
          walletTrader1,
          api.events.pablo.Swapped.is,
          api.tx.pablo.swap(1337, { assetId: 131, amount: 1_000 }, { assetId: 1, amount: 10_000 }, false) // ToDo: Update poolId & amounts if necessary!
        )
      ]).catch(exc => exc);
      expect(err.toString()).to.contain("ToDo");
    });

    it("#4.12 I can not buy or swap with the minimum amount requested greater than the trade would give.", async function () {
      const exc = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.swap(
          hardCodedPool1.poolId,
          { assetId: 130, amount: 1_000 },
          { assetId: 4, amount: 2000_000 },
          false
        ) // ToDo: Update poolId & amounts if necessary!
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("pablo.CannotRespectMinimumRequested");
    });

    it("#4.13 I can buy a huge amount with very high slippage.", async function () {
      const baseAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.baseAssetId
      );
      const amount = baseAssetFundsCurrentlyInPoolsBeforeTx.free.sub(new BN(Pica(1).toString()));
      const {
        data: [
          resultPoolId,
          resultWho,
          resultBaseAsset,
          resultQuoteAsset,
          resultBaseAmount,
          resultQuoteAmount,
          resultFee
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.buy(hardCodedPool1.poolId, 130, { assetId: 4, amount: amount }, false)
      );
    });

    it("#4.14 I can swap a huge amount with very high slippage.", async function () {
      const quoteAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.quoteAssetId
      );
      const amount = quoteAssetFundsCurrentlyInPoolsBeforeTx.free.sub(new BN(Pica(1).toString()));
      const {
        data: [
          resultPoolId,
          resultWho,
          resultBaseAsset,
          resultQuoteAsset,
          resultBaseAmount,
          resultQuoteAmount,
          resultFee
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.swap(
          hardCodedPool1.poolId,
          { assetId: hardCodedPool1.quoteAssetId, amount: amount },
          {
            assetId: hardCodedPool1.baseAssetId,
            amount: 0
          },
          false
        )
      );
    });

    it("#4.17 I can buy in the pool with 0 fees & pay 0 fees.");

    it("#4.18 I can swap in the pool with 0 fees & pay 0 fees.");
  });

  describe("3. Removing liquidity", function () {
    it("#3.1  I can not remove more liquidity than the amount equivalent to my LP token amount.", async function () {
      const poolQuery = await api.query.pablo.pools(0);
      const lpTokenId = poolQuery.unwrap().asDualAssetConstantProduct.lpToken;
      const lpAmount = <OrmlTokensAccountData>await api.query.tokens.accounts(walletLpProvider1.publicKey, lpTokenId);
      const amountToRemove = lpAmount.free.muln(1.5);
      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(BigInt(amountToRemove.toString())),
        "130": Pica(BigInt(amountToRemove.toString()))
      });

      const exc = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityRemoved.is,
        api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, amountToRemove, assets)
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("pablo.CannotRespectMinimumRequested");
    });

    it.skip("#3.2  I can not remove liquidity amounts of 0.", async function () {
      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(0),
        "130": Pica(0)
      });

      const exc = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityRemoved.is,
        api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, 0, assets)
      ).catch(exc => exc);
      expect(exc.toString()).to.contain('"arithmetic":"DivisionByZero"');
    });

    it("#3.4  I can not remove liquidity from a pool by using the LP tokens of the different pool.", async function () {
      const poolQuery = await api.query.pablo.pools(0);
      const lpTokenId = poolQuery.unwrap().asDualAssetConstantProduct.lpToken;
      const lpTokenAccountInfo = await api.query.tokens.accounts(walletLpProvider2.publicKey, lpTokenId);
      const availableLpTokens = lpTokenAccountInfo.free;

      const assets = api.createType("BTreeMap<u128, u128>", {
        "1": Pica(0),
        "130": Pica(0)
      });

      const {
        data: [resultWho, resultPoolId, resultAssetsAmount, resultMintedLp]
      } = await sendAndWaitForSuccess(
        api,
        walletLpProvider2,
        api.events.pablo.LiquidityRemoved.is,
        // Parameters: poolId, assetsMap, minMintAmount, keepAlive
        api.tx.pablo.removeLiquidity(hardCodedPool2.poolId, availableLpTokens, assets) // ToDo: Update pool id w/ created pool!
      );
    });

    it("#3.3  I can remove liquidity based on LP tokens which were sent to me.", async function () {
      const poolQuery = await api.query.pablo.pools(0);
      const lpTokenId = poolQuery.unwrap().asDualAssetConstantProduct.lpToken;
      const lpTokenAccountInfo = await api.query.tokens.accounts(walletLpProvider2.publicKey, lpTokenId);
      const availableLpTokens = lpTokenAccountInfo.free;

      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(0),
        "130": Pica(0)
      });

      const expectedAmountLpTokens = await api.rpc.pablo.simulateAddLiquidity(
        walletLpProvider2.address,
        hardCodedPool1.poolId.toString(), // ToDo: Update pool id w/ created pool!
        api.createType("BTreeMap<u128, u128>", assets)
      );

      const {
        data: [resultWho, resultPoolId, resultAssetsAmount, resultMintedLp]
      } = await sendAndWaitForSuccess(
        api,
        walletLpProvider2,
        api.events.pablo.LiquidityRemoved.is,
        // Parameters: poolId, assetsMap, minMintAmount, keepAlive
        api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, availableLpTokens, assets) // ToDo: Update pool id w/ created pool!
      );
      // ToDo

      expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletLpProvider2.publicKey).toString());
      expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPool1.poolId)); // ToDo: Update pool id w/ created pool!
      // expect(new BN(resultMintedLp.toString())).to.be.bignumber.closeTo(new BN(expectedAmountLpTokens.toString()), 1000)
      console.debug("MintedLp", resultMintedLp);
      console.debug("ExpectedLp", expectedAmountLpTokens);
    });

    it("#3.5  I can remove earlier provided liquidity.", async function () {
      const poolQuery = await api.query.pablo.pools(0);
      const lpTokenId = poolQuery.unwrap().asDualAssetConstantProduct.lpToken;
      const lpTokenAccountInfo = await api.query.tokens.accounts(walletLpProvider1.publicKey, lpTokenId);
      const availableLpTokens = lpTokenAccountInfo.free;

      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(0),
        "130": Pica(0)
      });

      const expectedAmountLpTokens = await api.rpc.pablo.simulateAddLiquidity(
        walletLpProvider1.address,
        hardCodedPool1.poolId.toString(), // ToDo: Update pool id w/ created pool!
        api.createType("BTreeMap<u128, u128>", assets)
      );

      const {
        data: [resultWho, resultPoolId, resultAssetsAmount, resultMintedLp]
      } = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityRemoved.is,
        // Parameters: poolId, assetsMap, minMintAmount, keepAlive
        api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, availableLpTokens.muln(0.1), assets) // ToDo: Update pool id w/ created pool!
      );
      // ToDo

      expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletLpProvider1.publicKey).toString());
      expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPool1.poolId)); // ToDo: Update pool id w/ created pool!
      // BTreeMaps are really the worst to deal with in JS... -.-
      assets.forEach(function(asset) {
        console.debug(asset.toHuman());
      });
      // expect(
      //   new BN(
      //     _.filter(assets, function () {
      //       console.debug(assets.entries().next().value[1]);
      //       if (assets.keys().next().value.toString() == "4") return assets.entries().next().value[1];
      //     }).toString()
      //   )
      // ).to.be.bignumber.greaterThan(0);
      // expect(
      //   new BN(
      //     _.filter(assets, function () {
      //       if (assets.keys().next().value.toString() == "130") return assets.entries().next().value[1];
      //     }).toString()
      //   )
      // ).to.be.bignumber.greaterThan(0);
      // expect(new BN(resultMintedLp.toString())).to.be.bignumber.closeTo(new BN(expectedAmountLpTokens.toString()), 1000)
      console.debug("MintedLp", resultMintedLp);
      console.debug("ExpectedLp", expectedAmountLpTokens);
    });

    it("#3.6  I can remove earlier provided liquidity with defined `minReceive`.", async function () {
      const poolQuery = await api.query.pablo.pools(0);
      const lpTokenId = poolQuery.unwrap().asDualAssetConstantProduct.lpToken;
      const receivingWallet = walletLpProvider2.publicKey;
      const lpAmount = <OrmlTokensAccountData>await api.query.tokens.accounts(walletLpProvider1.publicKey, lpTokenId);
      const amountLpToRemove = lpAmount.free;
      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(1),
        "130": Pica(10)
      });

      const expectLpAmount = await api.rpc.pablo.simulateRemoveLiquidity(
        walletLpProvider1.address,
        hardCodedPool1.poolId.toString(),
        amountLpToRemove.toString(),
        api.createType("BTreeMap<u128, u128>", assets)
      );
      const {
        data: [resultWho, resultPoolId, resultBaseAmount, resultQuoteAmount, resultTotalIssuance]
      } = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityRemoved.is,
        api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, amountLpToRemove, assets)
      );
      // ToDo
      expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletLpProvider1.publicKey).toString());
      expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPool1.poolId.toString()));
    });
  });
});
