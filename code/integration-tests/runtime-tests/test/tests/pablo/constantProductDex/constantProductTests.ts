import pabloTestConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { mintAssetsToWallet, Pica, USDT } from "@composable/utils/mintingHelper";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { ApiPromise } from "@polkadot/api";
import { sendAndWaitForSuccess, waitForBlocks } from "@composable/utils/polkadotjs";
import { expect } from "chai";
import BN from "bn.js";
import { ComposableTraitsDexFee, OrmlTokensAccountData } from "@composable/types/interfaces";
import {
  calculateInGivenOut,
  calculateOutGivenIn
} from "@composabletests/tests/pablo/testHandlers/constantProduct/weightedMath";
import BigNumber from "bignumber.js";
import { IEvent } from "@polkadot/types/types";
import { u128 } from "@polkadot/types-codec";
import { AccountId32 } from "@polkadot/types/interfaces/runtime";

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

const DEFAULT_LIQUIDITY_AMOUNT_TO_ADD = Pica(10_000);

async function verifyBuySwapOperation(
  api: ApiPromise,
  desiredAmount: bigint,
  tradingWallet: KeyringPair,
  txResult: IEvent<
    [
      poolId: u128,
      who: AccountId32,
      baseAsset: u128,
      quoteAsset: u128,
      baseAmount: u128,
      quoteAmount: u128,
      fee: ComposableTraitsDexFee
    ],
    unknown
  >,
  baseAssetFundsCurrentlyInPoolsBeforeTx: OrmlTokensAccountData,
  quoteAssetFundsCurrentlyInPoolsBeforeTx: OrmlTokensAccountData,
  baseAssetTraderFundsBeforeTx: OrmlTokensAccountData,
  quoteAssetTraderFundsBeforeTx: OrmlTokensAccountData,
  operationType: "swap" | "buy",
  hardCodedPoolData: {
    poolId: number;
    baseAssetId: number;
    quoteAssetId: number;
    lpTokenId: number;
    poolWalletAddress: string;
  }
) {
  const resultPoolId = txResult.data[0];
  const resultWho = txResult.data[1];
  const resultBaseAsset = txResult.data[2];
  const resultQuoteAsset = txResult.data[3];
  const resultBaseAmount = txResult.data[4];
  const resultQuoteAmount = txResult.data[5];
  const resultFee = txResult.data[6];

  // Getting pool liquidity after tx
  const baseAssetFundsCurrentlyInPoolsAfterTx = <OrmlTokensAccountData>(
    await api.query.tokens.accounts(hardCodedPoolData.poolWalletAddress, resultBaseAsset)
  );
  const quoteAssetFundsCurrentlyInPoolsAfterTx = <OrmlTokensAccountData>(
    await api.query.tokens.accounts(hardCodedPoolData.poolWalletAddress, resultQuoteAsset)
  );

  // Getting user funds
  const baseAssetTraderFundsAfterTx = <OrmlTokensAccountData>(
    await api.query.tokens.accounts(tradingWallet.publicKey, hardCodedPoolData.baseAssetId)
  );
  const quoteAssetTraderFundsAfterTx = <OrmlTokensAccountData>(
    await api.query.tokens.accounts(tradingWallet.publicKey, hardCodedPoolData.quoteAssetId)
  );

  expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPoolData.poolId.toString()));
  expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", tradingWallet.publicKey).toString());
  expect(resultBaseAsset).to.be.bignumber.equal(new BN(hardCodedPoolData.baseAssetId));
  expect(resultQuoteAsset).to.be.bignumber.equal(new BN(hardCodedPoolData.quoteAssetId));

  let expectedAmount: BigNumber;
  let expectedReducedByFee: BigNumber;

  // Get expected amounts & verifying for the desired operation.
  if (operationType == "swap") {
    expectedAmount = calculateOutGivenIn(
      BigNumber(baseAssetFundsCurrentlyInPoolsBeforeTx.free.toString()),
      BigNumber(quoteAssetFundsCurrentlyInPoolsBeforeTx.free.toString()),
      BigNumber(desiredAmount.toString()).minus(
        BigNumber(desiredAmount.toString()).dividedBy(100).multipliedBy(0.3) // Subtracting fee
      ),
      BigNumber(5),
      BigNumber(5)
    );

    expect(resultBaseAmount).to.be.bignumber.equal(
      new BN(expectedAmount.toFixed(0, 1)) // Sub fee?
    );
    expect(baseAssetFundsCurrentlyInPoolsAfterTx.free).to.be.bignumber.equal(
      baseAssetFundsCurrentlyInPoolsBeforeTx.free.sub(new BN(expectedAmount.toFixed(0, 1))) // New bug position
    );
  } else {
    expectedAmount = calculateInGivenOut(
      BigNumber(baseAssetFundsCurrentlyInPoolsBeforeTx.free.toString()),
      BigNumber(quoteAssetFundsCurrentlyInPoolsBeforeTx.free.toString()),
      BigNumber(desiredAmount.toString()),
      BigNumber(5),
      BigNumber(5)
    );
    expectedReducedByFee = BigNumber(expectedAmount.plus(BigNumber(resultFee.fee.toString())).toFixed(0));
    expect(resultQuoteAmount).to.be.bignumber.closeTo(new BN(expectedReducedByFee.toString()), new BN(1));
    expect(quoteAssetFundsCurrentlyInPoolsAfterTx.free).to.be.bignumber.closeTo(
      quoteAssetFundsCurrentlyInPoolsBeforeTx.free.add(new BN(expectedReducedByFee.toString())),
      new BN(1)
    );
  }
  // Verifying pool funds
  expect(baseAssetFundsCurrentlyInPoolsAfterTx.free).to.be.bignumber.equal(
    baseAssetFundsCurrentlyInPoolsBeforeTx.free.sub(resultBaseAmount)
  );

  // Verifying user funds
  expect(baseAssetTraderFundsAfterTx.free).to.be.bignumber.equal(
    baseAssetTraderFundsBeforeTx.free.add(resultBaseAmount)
  );
  expect(quoteAssetTraderFundsAfterTx.free).to.be.bignumber.equal(
    quoteAssetTraderFundsBeforeTx.free.sub(resultQuoteAmount)
  );
  return { baseAmount: resultBaseAmount, quoteAmount: resultQuoteAmount };
}

describe("[SHORT] Pablo: Constant Product Test Suite", function () {
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
    walletLpProvider3: KeyringPair,
    walletTrader1: KeyringPair;
  let poolId1: number, poolId2: number, poolId3: number;

  before("Initialize variables", async function () {
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice, devWalletFerdie } = getDevWallets(newKeyring);

    sudoKey = devWalletAlice;
    poolOwnerWallet = devWalletFerdie.derive("/test/pablo/pool/owner");
    walletLpProvider1 = devWalletFerdie.derive("/test/pablo/lp/provider/1");
    walletLpProvider2 = devWalletFerdie.derive("/test/pablo/lp/provider/2");
    walletLpProvider3 = devWalletFerdie.derive("/test/pablo/lp/provider/3");
    walletTrader1 = devWalletFerdie.derive("/test/pablo/trader/1");
  });

  before("Minting assets", async function () {
    // ToDo: Enable following when pool creation works!
    // await mintAssetsToWallet(api, poolOwnerWallet, sudoKey, [1]);
    await mintAssetsToWallet(
      api,
      walletLpProvider1,
      sudoKey,
      [1, 4, 130],
      10000000000000n * DEFAULT_LIQUIDITY_AMOUNT_TO_ADD
    );
    await mintAssetsToWallet(
      api,
      walletLpProvider2,
      sudoKey,
      [1, 4, 130],
      10000000000000n * DEFAULT_LIQUIDITY_AMOUNT_TO_ADD
    );
    await mintAssetsToWallet(
      api,
      walletLpProvider3,
      sudoKey,
      [1, 4, 130],
      10000000000000n * DEFAULT_LIQUIDITY_AMOUNT_TO_ADD
    );
    await mintAssetsToWallet(api, walletTrader1, sudoKey, [1, 4, 130]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  // ToDo: Re- enable and modify tests as soon as pool creation works!
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
        data: [resultPoolId, resultOwner]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.pablo.PoolCreated.is,
        api.tx.sudo.sudo(api.tx.pablo.create(poolConfiguration))
      );
      poolId1 = resultPoolId.toNumber();
      expect(resultOwner.toString()).to.be.equal(owner.toString());
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
        data: [resultPoolId, resultOwner]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.pablo.PoolCreated.is,
        api.tx.sudo.sudo(api.tx.pablo.create(poolConfiguration))
      );
      poolId2 = resultPoolId.toNumber();
      expect(resultOwner.toString()).to.be.equal(owner.toString());
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
        data: [resultPoolId, resultOwner]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.pablo.PoolCreated.is,
        api.tx.sudo.sudo(api.tx.pablo.create(poolConfiguration))
      );
      poolId3 = resultPoolId.toNumber();
      expect(resultOwner.toString()).to.be.equal(owner.toString());
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
            "1": 500_000 // Tbh. may not even do smth. cause that entry might override the above one.
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
      const ksmAmount = Pica(10_000);
      const usdtAmount = USDT(100_000);
      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": ksmAmount,
        "130": usdtAmount
      });

      const lpTokenFundsInWalletBefore = await api.query.tokens.accounts(
        walletLpProvider1.publicKey,
        hardCodedPool1.lpTokenId
      );
      const expectedAmountLpTokens = await api.rpc.pablo.simulateAddLiquidity(
        walletLpProvider1.address,
        hardCodedPool1.poolId.toString(),
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

      const {
        data: [resultWho, resultPoolId, resultAssetsAmount, resultMintedLp]
      } = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityAdded.is,
        // Parameters: poolId, assetsMap, minMintAmount, keepAlive
        api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assets, 0, true)
      );

      const lpTokenFundsInWalletAfter = await api.query.tokens.accounts(
        walletLpProvider1.publicKey,
        hardCodedPool1.lpTokenId
      );
      const receivedLpTokens = lpTokenFundsInWalletAfter.free.sub(lpTokenFundsInWalletBefore.free);
      const baseAssetFundsCurrentlyInPoolsAfter = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.baseAssetId
      );
      const quoteAssetFundsCurrentlyInPoolsAfter = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.quoteAssetId
      );

      expect(baseAssetFundsCurrentlyInPoolsAfter.free).to.be.bignumber.equal(
        baseAssetFundsCurrentlyInPoolsBefore.free.add(new BN(ksmAmount.toString()))
      );
      expect(quoteAssetFundsCurrentlyInPoolsAfter.free).to.be.bignumber.equal(
        quoteAssetFundsCurrentlyInPoolsBefore.free.add(new BN(usdtAmount.toString()))
      );
      expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletLpProvider1.publicKey).toString());
      expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPool1.poolId));
      expect(resultAssetsAmount).to.be.eql(assets);
      expect(new BN(expectedAmountLpTokens.toString()))
        .to.be.bignumber.equal(receivedLpTokens)
        .to.be.bignumber.equal(resultMintedLp);
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
      // Can't use `expect[..].bignumber.equal` here bc. it's configured for BN.js not BigNumber.js
      expect(BigNumber(resultAmount.toString()).isEqualTo(BigNumber(amountToTransfer.toString()))).to.be.true;
    });

    it("#2.4  I can not add liquidity amounts of 0.", async function () {
      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(0),
        "130": USDT(0)
      });
      const exc = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityAdded.is,
        api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assets, 0, true)
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("pablo.InvalidAmount");
    });

    it(
      "#2.5  When adding liquidity without respecting the pools ratio, " +
        "the amounts will be adjusted according to the current pool ratio.",
      async function () {
        // ToDo: Might want to add checks to verify the event reports the actual amounts.
        // Transaction #1
        let baseAssetFundsCurrentlyInPoolsBefore = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.baseAssetId
        );
        let quoteAssetFundsCurrentlyInPoolsBefore = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.quoteAssetId
        );
        const assets = api.createType("BTreeMap<u128, u128>", {
          "4": Pica(1),
          "130": USDT(99999999)
        });
        await sendAndWaitForSuccess(
          api,
          walletLpProvider1,
          api.events.pablo.LiquidityAdded.is,
          api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assets, 0, true)
        );
        expect(
          BigNumber(baseAssetFundsCurrentlyInPoolsBefore.free.toString())
            .dividedBy(BigNumber(quoteAssetFundsCurrentlyInPoolsBefore.free.toString()))
            .toNumber()
        ).to.be.equal(Number(100_000)); // 100k is the ratio due to USDT having 6 decimals & PICA/KSM having 12.

        // Transaction #2
        baseAssetFundsCurrentlyInPoolsBefore = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.baseAssetId
        );
        quoteAssetFundsCurrentlyInPoolsBefore = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.quoteAssetId
        );
        const assets2 = api.createType("BTreeMap<u128, u128>", {
          "4": Pica(50),
          "130": USDT(50)
        });
        await sendAndWaitForSuccess(
          api,
          walletLpProvider1,
          api.events.pablo.LiquidityAdded.is,
          api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assets2, 0, true)
        );
        expect(
          BigNumber(baseAssetFundsCurrentlyInPoolsBefore.free.toString())
            .dividedBy(BigNumber(quoteAssetFundsCurrentlyInPoolsBefore.free.toString()))
            .toNumber()
        ).to.be.equal(100_000); // 100k is the ratio due to USDT having 6 decimals & PICA/KSM having 12.

        // Transaction #3
        baseAssetFundsCurrentlyInPoolsBefore = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.baseAssetId
        );
        quoteAssetFundsCurrentlyInPoolsBefore = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.quoteAssetId
        );
        const assets3 = api.createType("BTreeMap<u128, u128>", {
          "4": Pica(9999),
          "130": USDT(1)
        });
        await sendAndWaitForSuccess(
          api,
          walletLpProvider1,
          api.events.pablo.LiquidityAdded.is,
          api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assets3, 0, true)
        );
        expect(
          BigNumber(baseAssetFundsCurrentlyInPoolsBefore.free.toString())
            .dividedBy(BigNumber(quoteAssetFundsCurrentlyInPoolsBefore.free.toString()))
            .toNumber()
        ).to.be.equal(100_000); // 100k is the ratio due to USDT having 6 decimals & PICA/KSM having 12.

        // Transaction #4
        baseAssetFundsCurrentlyInPoolsBefore = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.baseAssetId
        );
        quoteAssetFundsCurrentlyInPoolsBefore = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.quoteAssetId
        );
        const assets4 = api.createType("BTreeMap<u128, u128>", {
          "4": Pica(33),
          "130": USDT(67)
        });
        await sendAndWaitForSuccess(
          api,
          walletLpProvider1,
          api.events.pablo.LiquidityAdded.is,
          api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assets4, 0, true)
        );
        expect(
          BigNumber(baseAssetFundsCurrentlyInPoolsBefore.free.toString())
            .dividedBy(BigNumber(quoteAssetFundsCurrentlyInPoolsBefore.free.toString()))
            .toNumber()
        ).to.be.equal(100_000); // 100k is the ratio due to USDT having 6 decimals & PICA/KSM having 12.
      }
    );
  });

  describe("4. Trading pt. 1", function () {
    it("#4.9  I can not buy in a pool without liquidity.", async function () {
      const exc = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.buy(
          hardCodedPool2.poolId,
          hardCodedPool2.quoteAssetId,
          {
            assetId: hardCodedPool2.baseAssetId,
            amount: 10_000
          },
          false
        )
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("WRONG ERROR MESSAGE! Expected: pablo.CannotRespectMinimumRequested");
    });

    it("#4.10 I can not swap in a pool without liquidity.", async function () {
      const exc = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.swap(
          hardCodedPool2.poolId,
          {
            assetId: hardCodedPool2.quoteAssetId,
            amount: 1_000
          },
          { assetId: hardCodedPool2.baseAssetId, amount: 10_000 },
          false
        )
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("pablo.CannotRespectMinimumRequested");
    });
  });

  describe("2. Providing liquidity pt. 2", function () {
    it("#2.6  I can add liquidity with a defined `minMintAmount`.", async function () {
      const assets = api.createType("BTreeMap<u128, u128>", {
        "1": Pica(10_000),
        "130": USDT(100_000)
      });

      const expectedAmountLpTokens = await api.rpc.pablo.simulateAddLiquidity(
        walletLpProvider1.address,
        hardCodedPool2.poolId.toString(),
        api.createType("BTreeMap<u128, u128>", assets)
      );
      const lpTokenFundsInWalletBefore = await api.query.tokens.accounts(
        walletLpProvider1.publicKey,
        hardCodedPool2.lpTokenId
      );

      const {
        data: [resultWho, resultPoolId, resultAssetsAmount]
      } = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityAdded.is,
        api.tx.pablo.addLiquidity(hardCodedPool2.poolId, assets, lpTokenFundsInWalletBefore.free, true)
      );
      const lpTokenFundsInWalletAfter = await api.query.tokens.accounts(
        walletLpProvider1.publicKey,
        hardCodedPool2.lpTokenId
      );
      const receivedLpTokens = lpTokenFundsInWalletAfter.free.sub(lpTokenFundsInWalletBefore.free);

      expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletLpProvider1.publicKey).toString());
      expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPool2.poolId));
      expect(resultAssetsAmount).to.be.eql(assets);
      expect(new BN(expectedAmountLpTokens.toString())).to.be.bignumber.equal(receivedLpTokens);
    });

    it("#2.7  I can add liquidity to a pool with already available liquidity.", async function () {
      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(10_000),
        "130": USDT(100_000)
      });

      const expectedAmountLpTokens = await api.rpc.pablo.simulateAddLiquidity(
        walletLpProvider1.address,
        hardCodedPool1.poolId.toString(),
        api.createType("BTreeMap<u128, u128>", assets)
      );
      const lpTokenFundsInWalletBefore = await api.query.tokens.accounts(
        walletLpProvider1.publicKey,
        hardCodedPool1.lpTokenId
      );

      const {
        data: [resultWho, resultPoolId, resultAssetsAmount, resultMintedLp]
      } = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityAdded.is,
        api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assets, 0, true)
      );
      const lpTokenFundsInWalletAfter = await api.query.tokens.accounts(
        walletLpProvider1.publicKey,
        hardCodedPool1.lpTokenId
      );
      const receivedLpTokens = lpTokenFundsInWalletAfter.free.sub(lpTokenFundsInWalletBefore.free);

      expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletLpProvider1.publicKey).toString());
      expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPool1.poolId));
      expect(resultAssetsAmount).to.be.eql(assets);
      expect(new BN(expectedAmountLpTokens.toString()))
        .to.be.bignumber.equal(receivedLpTokens)
        .to.be.bignumber.equal(resultMintedLp);
    });
  });

  describe("4. Trading pt. 2", function () {
    it("#4.1  I can not buy an amount more than available liquidity.", async function () {
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
          { assetId: 130, amount: USDT(9999999999999999999n) },
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
        const amountUsdt = USDT(100);

        // Getting funds in pool before tx
        const baseAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.baseAssetId
        );
        const quoteAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.quoteAssetId
        );

        // Getting user funds before tx
        const baseAssetTraderFundsBeforeTx = await api.query.tokens.accounts(
          walletTrader1.publicKey,
          hardCodedPool1.baseAssetId
        );
        const quoteAssetTraderFundsBeforeTx = await api.query.tokens.accounts(
          walletTrader1.publicKey,
          hardCodedPool1.quoteAssetId
        );

        const pricesForResult = await api.rpc.pablo.pricesFor(
          hardCodedPool1.poolId.toString(),
          hardCodedPool1.quoteAssetId.toString(),
          hardCodedPool1.baseAssetId.toString(),
          amountUsdt.toString()
        );
        const pricesForResultInverted = await api.rpc.pablo.pricesFor(
          hardCodedPool1.poolId.toString(),
          hardCodedPool1.baseAssetId.toString(),
          hardCodedPool1.quoteAssetId.toString(),
          amountUsdt.toString()
        );
        const txResult = await sendAndWaitForSuccess(
          api,
          walletTrader1,
          api.events.pablo.Swapped.is,
          api.tx.pablo.swap(
            hardCodedPool1.poolId,
            { assetId: hardCodedPool1.quoteAssetId, amount: amountUsdt },
            {
              assetId: hardCodedPool1.baseAssetId,
              amount: 0
            },
            false
          )
        );
        const { baseAmount, quoteAmount } = await verifyBuySwapOperation(
          api,
          amountUsdt,
          walletTrader1,
          txResult,
          baseAssetFundsCurrentlyInPoolsBeforeTx,
          quoteAssetFundsCurrentlyInPoolsBeforeTx,
          baseAssetTraderFundsBeforeTx,
          quoteAssetTraderFundsBeforeTx,
          "swap",
          hardCodedPool1
        );
      }
    );

    it(
      "#4.5  I can buy an amount, and provided by the amount i want to get out, " +
        "and it'll be adjusted by the `inGivenOut` formula.",
      async function () {
        const amount = Pica(100);

        // Getting funds in pool before tx
        const baseAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.baseAssetId
        );
        const quoteAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.quoteAssetId
        );

        // Getting user funds before tx
        const baseAssetTraderFundsBeforeTx = await api.query.tokens.accounts(
          walletTrader1.publicKey,
          hardCodedPool1.baseAssetId
        );
        const quoteAssetTraderFundsBeforeTx = await api.query.tokens.accounts(
          walletTrader1.publicKey,
          hardCodedPool1.quoteAssetId
        );
        const pricesForResult = await api.rpc.pablo.pricesFor(
          hardCodedPool1.poolId.toString(),
          hardCodedPool1.baseAssetId.toString(),
          hardCodedPool1.quoteAssetId.toString(),
          amount.toString()
        );
        const pricesForResultInverted = await api.rpc.pablo.pricesFor(
          hardCodedPool1.poolId.toString(),
          hardCodedPool1.quoteAssetId.toString(),
          hardCodedPool1.baseAssetId.toString(),
          amount.toString()
        );

        const txResult = await sendAndWaitForSuccess(
          api,
          walletTrader1,
          api.events.pablo.Swapped.is,
          api.tx.pablo.buy(hardCodedPool1.poolId, 130, { assetId: 4, amount: amount }, false)
        );
        const { baseAmount, quoteAmount } = await verifyBuySwapOperation(
          api,
          amount,
          walletTrader1,
          txResult,
          baseAssetFundsCurrentlyInPoolsBeforeTx,
          quoteAssetFundsCurrentlyInPoolsBeforeTx,
          baseAssetTraderFundsBeforeTx,
          quoteAssetTraderFundsBeforeTx,
          "buy",
          hardCodedPool1
        );
      }
    );

    it("#4.6  I can not buy 0 amounts of any asset.", async function () {
      const {
        data: [resultPoolId, resultWho, resultBaseAsset, resultQuoteAsset, resultBaseAmount, resultQuoteAmount]
      } = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.buy(hardCodedPool1.poolId, 130, { assetId: 4, amount: 0 }, false)
      );
      expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPool1.poolId.toString()));
      expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletTrader1.publicKey).toString());
      expect(resultBaseAsset).to.be.bignumber.equal(new BN(hardCodedPool1.baseAssetId));
      expect(resultQuoteAsset).to.be.bignumber.equal(new BN(hardCodedPool1.quoteAssetId));
      expect(resultBaseAmount).to.be.bignumber.equal(new BN(0));
      expect(resultQuoteAmount).to.be.bignumber.equal(new BN(0));
    });

    it("#4.19  I can not swap 0 amounts of any asset.", async function () {
      const {
        data: [resultPoolId, resultWho, resultBaseAsset, resultQuoteAsset, resultBaseAmount, resultQuoteAmount]
      } = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.swap(hardCodedPool1.poolId, { assetId: 130, amount: 0 }, { assetId: 4, amount: 0 }, false)
      );
      expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPool1.poolId.toString()));
      expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletTrader1.publicKey).toString());
      expect(resultBaseAsset).to.be.bignumber.equal(new BN(hardCodedPool1.baseAssetId));
      expect(resultQuoteAsset).to.be.bignumber.equal(new BN(hardCodedPool1.quoteAssetId));
      expect(resultBaseAmount).to.be.bignumber.equal(new BN(0));
      expect(resultQuoteAmount).to.be.bignumber.equal(new BN(0));
    });

    it("#4.7  I can not buy all of the available liquidity of a pool.", async function () {
      const baseAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.baseAssetId
      );

      const exc = await sendAndWaitForSuccess(
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
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("Currently buggy! Throws wrong error message.");
    });

    it("#4.8  I can not buy with the base asset being the same as the quote asset.", async function () {
      const transferAmount = Pica(100);

      const baseAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.baseAssetId
      );
      const Bo = BigNumber(baseAssetFundsCurrentlyInPoolsBeforeTx.free.toString());
      const Bi = BigNumber(baseAssetFundsCurrentlyInPoolsBeforeTx.free.toString());
      const priceFirstTrade = calculateInGivenOut(
        Bo,
        Bi,
        BigNumber(transferAmount.toString()),
        BigNumber(5),
        BigNumber(5)
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
          hardCodedPool1.baseAssetId,
          {
            assetId: hardCodedPool1.baseAssetId,
            amount: transferAmount
          },
          false
        )
      );

      expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPool1.poolId));
      expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletTrader1.publicKey).toString());
      expect(resultBaseAsset).to.be.bignumber.equal(new BN(hardCodedPool1.baseAssetId));
      expect(resultQuoteAsset).to.be.bignumber.equal(new BN(hardCodedPool1.baseAssetId));
      expect(resultBaseAmount).to.be.bignumber.equal(new BN(transferAmount.toString()));
      expect(resultQuoteAmount).to.be.bignumber.equal(
        new BN(priceFirstTrade.toFixed(0)).add(resultFee.lpFee.add(resultFee.protocolFee))
      );
    });

    it("#4.20  I can not swap with the minimum amount requested being the same as the inAsset.", async function () {
      // Arbitrary wait because the last test will fail immediately, and can cause this test to run
      // into priority too low issues.
      await waitForBlocks(api);
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

    it("#4.11 I can not buy or swap in a pool that doesn't exist.", async function () {
      // Arbitrary wait because the last test will fail immediately, and can cause this test to run
      // into priority too low issues.
      await waitForBlocks(api);
      const err = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.buy(1337, 131, { assetId: 1, amount: 10_000 }, false)
      ).catch(exc => exc);
      expect(err.toString()).to.contain("pablo.PoolNotFound");
      const err2 = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.pablo.Swapped.is,
        api.tx.pablo.swap(1337, { assetId: 131, amount: 1_000 }, { assetId: 1, amount: 10_000 }, false)
      ).catch(exc => exc);
      expect(err2.toString()).to.contain("pablo.PoolNotFound");
    });

    it(
      "#4.12 I can buy or swap with the minimum amount requested greater than the trade would give, " +
        "but the amounts will be automatically adjusted to the pools ratio.",
      async function () {
        const exc = await sendAndWaitForSuccess(
          api,
          walletTrader1,
          api.events.pablo.Swapped.is,
          api.tx.pablo.swap(
            hardCodedPool1.poolId,
            { assetId: hardCodedPool1.quoteAssetId, amount: 10 },
            { assetId: hardCodedPool1.baseAssetId, amount: 2000_000 },
            false
          )
        ).catch(exc => exc);
        expect(exc.toString()).to.contain("pablo.CannotRespectMinimumRequested");
      }
    );

    it("#4.13 I can buy a huge amount with very high slippage.", async function () {
      this.timeout(10 * 60 * 1000);
      before(async function () {
        await mintAssetsToWallet(api, walletTrader1, sudoKey, [4, 130], BigInt(10 ** 4194304));
        await mintAssetsToWallet(api, walletTrader1, sudoKey, [4, 130], BigInt(Number.MAX_SAFE_INTEGER));
        await mintAssetsToWallet(api, walletTrader1, sudoKey, [4, 130], BigInt(Number.MAX_SAFE_INTEGER));
        await mintAssetsToWallet(api, walletTrader1, sudoKey, [4, 130], BigInt(Number.MAX_SAFE_INTEGER));
      });
      const baseAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.baseAssetId
      );
      const amount = baseAssetFundsCurrentlyInPoolsBeforeTx.free.sub(new BN(Pica(10_000).toString()));
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
      const amount = quoteAssetFundsCurrentlyInPoolsBeforeTx.free.sub(new BN(USDT(1).toString()));
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
        "130": USDT(BigInt(amountToRemove.toString()))
      });

      const exc = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityRemoved.is,
        api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, amountToRemove, assets)
      ).catch(exc => exc);
      expect(exc.toString()).to.contain("pablo.CannotRespectMinimumRequested");
    });

    it("#3.2  I can not remove liquidity amounts of 0.", async function () {
      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(0),
        "130": USDT(0)
      });

      const {
        data: [resultWho, resultPoolId, resultAssetsAmount]
      } = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityRemoved.is,
        api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, 0, assets)
      );
      expect(resultAssetsAmount.toString()).to.be.equal('{"4":0,"130":0}');
    });

    it("#3.4  I can not remove liquidity from a pool by using the LP tokens of the different pool.", async function () {
      const poolQuery = await api.query.pablo.pools(0);
      const lpTokenId = poolQuery.unwrap().asDualAssetConstantProduct.lpToken;
      const lpTokenAccountInfo = await api.query.tokens.accounts(walletLpProvider2.publicKey, lpTokenId);
      const availableLpTokens = lpTokenAccountInfo.free;

      const assets = api.createType("BTreeMap<u128, u128>", {
        "1": Pica(0),
        "130": USDT(0)
      });

      const exc = await sendAndWaitForSuccess(
        api,
        walletLpProvider2,
        api.events.pablo.LiquidityRemoved.is,
        // Parameters: poolId, assetsMap, minMintAmount, keepAlive
        api.tx.pablo.removeLiquidity(hardCodedPool2.poolId, availableLpTokens, assets)
      ).catch(exc => exc);
      expect(exc.toString()).to.contain('{"token":"NoFunds"}');
    });

    it("#3.3  I can remove liquidity based on LP tokens which were sent to me.", async function () {
      const poolQuery = await api.query.pablo.pools(0);
      const lpTokenId = poolQuery.unwrap().asDualAssetConstantProduct.lpToken;
      const lpTokenAccountInfo = await api.query.tokens.accounts(walletLpProvider2.publicKey, lpTokenId);
      const availableLpTokens = lpTokenAccountInfo.free;

      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(0),
        "130": USDT(0)
      });
      const expectedAmountReturnedFunds = await api.rpc.pablo.simulateRemoveLiquidity(
        walletLpProvider2.address,
        hardCodedPool1.poolId.toString(),
        availableLpTokens.toString(),
        api.createType("BTreeMap<SafeRpcWrapper, SafeRpcWrapper>", assets)
      );

      const {
        data: [resultWho, resultPoolId, resultAssetsAmount, resultMintedLp]
      } = await sendAndWaitForSuccess(
        api,
        walletLpProvider2,
        api.events.pablo.LiquidityRemoved.is,
        // Parameters: poolId, assetsMap, minMintAmount, keepAlive
        api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, availableLpTokens, assets)
      );
      expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletLpProvider2.publicKey).toString());
      expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPool1.poolId));
      // @ts-ignore
      expect(resultAssetsAmount.toPrimitive()[4]).to.be.equal(expectedAmountReturnedFunds["assets"].toPrimitive()[4]);
      // @ts-ignore
      expect(resultAssetsAmount.toPrimitive()[130]).to.be.equal(
        // @ts-ignore
        expectedAmountReturnedFunds["assets"].toPrimitive()[130]
      );
    });

    it("#3.5  I can remove earlier provided liquidity.", async function () {
      const poolQuery = await api.query.pablo.pools(0);
      const lpTokenId = poolQuery.unwrap().asDualAssetConstantProduct.lpToken;
      const lpTokenAccountInfo = await api.query.tokens.accounts(walletLpProvider1.publicKey, lpTokenId);
      const availableLpTokens = lpTokenAccountInfo.free;
      const amountLpToRemove = availableLpTokens.muln(0.1);

      const baseAssetFundsInPoolsBeforeRemoval = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.baseAssetId
      );
      const quoteAssetFundsInPoolsBeforeRemoval = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.quoteAssetId
      );
      const poolFundsRatioBeforeRemoval = BigNumber(baseAssetFundsInPoolsBeforeRemoval.free.toString())
        .dividedBy(BigNumber(quoteAssetFundsInPoolsBeforeRemoval.free.toString()))
        .toNumber();

      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(0),
        "130": USDT(0)
      });

      const expectedReturnAmount = await api.rpc.pablo.simulateRemoveLiquidity(
        walletLpProvider1.address,
        hardCodedPool1.poolId.toString(),
        amountLpToRemove.toString(),
        api.createType("BTreeMap<SafeRpcWrapper, SafeRpcWrapper>", assets)
      );
      const {
        data: [resultWho, resultPoolId, resultAssetsAmount]
      } = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityRemoved.is,
        // Parameters: poolId, assetsMap, minMintAmount, keepAlive
        api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, availableLpTokens.muln(0.1), assets)
      );
      expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletLpProvider1.publicKey).toString());
      expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPool1.poolId));
      // @ts-ignore
      expect(expectedReturnAmount["assets"].toPrimitive()[4]).to.be.equal(resultAssetsAmount.toPrimitive()[4]);
      // @ts-ignore
      expect(expectedReturnAmount["assets"].toPrimitive()[130]).to.be.equal(resultAssetsAmount.toPrimitive()[130]);
      const baseAssetFundsInPoolsAfterRemoval = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.baseAssetId
      );
      const quoteAssetFundsInPoolsAfterRemoval = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.quoteAssetId
      );
      // @ts-ignore
      const returnedAmountBaseAsset = resultAssetsAmount.toPrimitive()[4];
      // @ts-ignore
      const returnedAmountQuoteAsset = resultAssetsAmount.toPrimitive()[130];
      expect(baseAssetFundsInPoolsAfterRemoval.free).to.be.bignumber.equal(
        baseAssetFundsInPoolsBeforeRemoval.free.sub(new BN(returnedAmountBaseAsset.toString()))
      );
      expect(quoteAssetFundsInPoolsAfterRemoval.free).to.be.bignumber.equal(
        quoteAssetFundsInPoolsBeforeRemoval.free.sub(new BN(returnedAmountQuoteAsset.toString()))
      );
      const poolFundsRatioAfterRemoval = BigNumber(baseAssetFundsInPoolsBeforeRemoval.free.toString())
        .dividedBy(BigNumber(quoteAssetFundsInPoolsBeforeRemoval.free.toString()))
        .toNumber();
      expect(poolFundsRatioAfterRemoval).to.be.equal(poolFundsRatioBeforeRemoval);
    });

    it("#3.6  I can remove earlier provided liquidity with defined `minReceive`.", async function () {
      const poolQuery = await api.query.pablo.pools(0);
      const lpTokenId = poolQuery.unwrap().asDualAssetConstantProduct.lpToken;
      const lpAmount = <OrmlTokensAccountData>await api.query.tokens.accounts(walletLpProvider1.publicKey, lpTokenId);
      const amountLpToRemove = lpAmount.free;
      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(1),
        "130": USDT(10)
      });
      const baseAssetFundsInPoolsBeforeRemoval = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.baseAssetId
      );
      const quoteAssetFundsInPoolsBeforeRemoval = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.quoteAssetId
      );
      const poolFundsRatioBeforeRemoval = BigNumber(baseAssetFundsInPoolsBeforeRemoval.free.toString())
        .dividedBy(BigNumber(quoteAssetFundsInPoolsBeforeRemoval.free.toString()))
        .toNumber();

      const expectedReturnAmount = await api.rpc.pablo.simulateRemoveLiquidity(
        walletLpProvider1.address,
        hardCodedPool1.poolId.toString(),
        amountLpToRemove.toString(),
        api.createType("BTreeMap<SafeRpcWrapper, SafeRpcWrapper>", assets)
      );
      const {
        data: [resultWho, resultPoolId, resultAssetsAmount]
      } = await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityRemoved.is,
        api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, amountLpToRemove, assets)
      );
      // ToDo
      expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletLpProvider1.publicKey).toString());
      expect(resultPoolId).to.be.bignumber.equal(new BN(hardCodedPool1.poolId.toString()));
      // @ts-ignore
      expect(expectedReturnAmount["assets"].toPrimitive()[4]).to.be.equal(resultAssetsAmount.toPrimitive()[4]);
      // @ts-ignore
      expect(expectedReturnAmount["assets"].toPrimitive()[130]).to.be.equal(resultAssetsAmount.toPrimitive()[130]);
      const baseAssetFundsInPoolsAfterRemoval = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.baseAssetId
      );
      const quoteAssetFundsInPoolsAfterRemoval = await api.query.tokens.accounts(
        hardCodedPool1.poolWalletAddress,
        hardCodedPool1.quoteAssetId
      );
      // @ts-ignore
      const returnedAmountBaseAsset = resultAssetsAmount.toPrimitive()[4];
      // @ts-ignore
      const returnedAmountQuoteAsset = resultAssetsAmount.toPrimitive()[130];
      expect(baseAssetFundsInPoolsAfterRemoval.free).to.be.bignumber.equal(
        baseAssetFundsInPoolsBeforeRemoval.free.sub(new BN(returnedAmountBaseAsset.toString()))
      );
      expect(quoteAssetFundsInPoolsAfterRemoval.free).to.be.bignumber.equal(
        quoteAssetFundsInPoolsBeforeRemoval.free.sub(new BN(returnedAmountQuoteAsset.toString()))
      );
      const poolFundsRatioAfterRemoval = BigNumber(baseAssetFundsInPoolsBeforeRemoval.free.toString())
        .dividedBy(BigNumber(quoteAssetFundsInPoolsBeforeRemoval.free.toString()))
        .toNumber();
      expect(poolFundsRatioAfterRemoval).to.be.equal(poolFundsRatioBeforeRemoval);
    });

    it(
      "#3.7 After adding funds, transferring 50% of them to another wallet " +
        "if there's been trades in the meantime, both wallets can withdraw their liquidity," +
        "and will receive equal amounts of funds back.",
      async function () {
        this.timeout(5 * 60 * 1000);
        /*
          1. Provide liquidity
          2. Move 50% of liquidity to another wallet.
          3. Trade
          4. Remove liquidity of first wallet (expect fees)
          5. Remove liquidity of second wallet (expect no fees)
        */

        // 1. Providing liquidity
        const lpAssetAmounts = api.createType("BTreeMap<u128, u128>", {
          "4": Pica(100_000),
          "130": USDT(100_000)
        });
        const {
          data: [resultLpAdditionWho, resultLpAdditionPoolId, resultLpAdditionAssetsAmount, resultLpAdditionMintedLp]
        } = await sendAndWaitForSuccess(
          api,
          walletLpProvider1,
          api.events.pablo.LiquidityAdded.is,
          api.tx.pablo.addLiquidity(hardCodedPool1.poolId, lpAssetAmounts, 0, true)
        );

        // 2. Transferring LP tokens
        const receivingWallet = walletLpProvider2.publicKey;
        const lpAmount = <OrmlTokensAccountData>(
          await api.query.tokens.accounts(walletLpProvider1.publicKey, hardCodedPool1.lpTokenId)
        );
        const amountToTransfer = lpAmount.free.muln(0.5);
        const {
          data: [resultCurrencyId, resultFrom, resultTo, resultAmount]
        } = await sendAndWaitForSuccess(
          api,
          walletLpProvider1,
          api.events.tokens.Transfer.is,
          api.tx.assets.transfer(hardCodedPool1.lpTokenId, receivingWallet, amountToTransfer, false)
        );

        // 3. Trade
        const transferAmount = Pica(100);
        // Getting funds in pool before tx
        const baseAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.baseAssetId
        );
        const quoteAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.quoteAssetId
        );
        // Getting user funds before tx
        const baseAssetTraderFundsBeforeTx = await api.query.tokens.accounts(
          walletTrader1.publicKey,
          hardCodedPool1.baseAssetId
        );
        const quoteAssetTraderFundsBeforeTx = await api.query.tokens.accounts(
          walletTrader1.publicKey,
          hardCodedPool1.quoteAssetId
        );
        const Bo = BigNumber(baseAssetFundsCurrentlyInPoolsBeforeTx.free.toString());
        const Bi = BigNumber(quoteAssetFundsCurrentlyInPoolsBeforeTx.free.toString());
        const priceFirstTrade = calculateInGivenOut(
          Bo,
          Bi,
          BigNumber(transferAmount.toString()),
          BigNumber(5),
          BigNumber(5)
        );
        const txResult = await sendAndWaitForSuccess(
          api,
          walletTrader1,
          api.events.pablo.Swapped.is,
          api.tx.pablo.buy(hardCodedPool1.poolId, 130, { assetId: 4, amount: transferAmount }, false)
        );
        await verifyBuySwapOperation(
          api,
          transferAmount,
          walletTrader1,
          txResult,
          baseAssetFundsCurrentlyInPoolsBeforeTx,
          quoteAssetFundsCurrentlyInPoolsBeforeTx,
          baseAssetTraderFundsBeforeTx,
          quoteAssetTraderFundsBeforeTx,
          "buy",
          hardCodedPool1
        );
        const amountEarnedFeesQuoteAssetWallet1 = BigNumber(txResult.data[6].fee.toString());

        const quoteAssetFundsLpProvider1BeforeRemoval = await api.query.tokens.accounts(
          walletLpProvider1.publicKey,
          hardCodedPool1.quoteAssetId
        );
        const quoteAssetFundsLpProvider2BeforeRemoval = await api.query.tokens.accounts(
          walletLpProvider2.publicKey,
          hardCodedPool1.quoteAssetId
        );

        // 4. Remove initial liquidity
        const lpTokenId = hardCodedPool1.lpTokenId;
        const lpTokenAccountInfo1 = await api.query.tokens.accounts(walletLpProvider1.publicKey, lpTokenId);
        const availableLpTokens1 = lpTokenAccountInfo1.free;

        const removeAssetsMinAmounts = api.createType("BTreeMap<u128, u128>", {
          "4": Pica(0),
          "130": USDT(0)
        });

        const expectedAmountLpTokens = await api.rpc.pablo.simulateAddLiquidity(
          walletLpProvider1.address,
          hardCodedPool1.poolId.toString(), // ToDo: Update pool id w/ created pool!
          api.createType("BTreeMap<u128, u128>", removeAssetsMinAmounts)
        );

        const {
          data: [resultWho, resultPoolId, resultAssetsAmount, resultMintedLp]
        } = await sendAndWaitForSuccess(
          api,
          walletLpProvider1,
          api.events.pablo.LiquidityRemoved.is,
          // Parameters: poolId, assetsMap, minMintAmount, keepAlive
          api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, availableLpTokens1, removeAssetsMinAmounts) // ToDo: Update pool id w/ created pool!
        );

        // 5. Remove 2nd wallet liquidity
        const lpTokenAccountInfo2 = await api.query.tokens.accounts(walletLpProvider2.publicKey, lpTokenId);
        const availableLpTokens2 = lpTokenAccountInfo2.free;

        await sendAndWaitForSuccess(
          api,
          walletLpProvider2,
          api.events.pablo.LiquidityRemoved.is,
          // Parameters: poolId, assetsMap, minMintAmount, keepAlive
          api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, availableLpTokens2, removeAssetsMinAmounts) // ToDo: Update pool id w/ created pool!
        );

        const finalQuoteAssetFundsLpProvider1 = await api.query.tokens.accounts(
          walletLpProvider1.publicKey,
          hardCodedPool1.quoteAssetId
        );
        const finalQuoteAssetFundsLpProvider2 = await api.query.tokens.accounts(
          walletLpProvider2.publicKey,
          hardCodedPool1.quoteAssetId
        );

        expect(
          finalQuoteAssetFundsLpProvider1.free
            .sub(quoteAssetFundsLpProvider1BeforeRemoval.free)
            .add(finalQuoteAssetFundsLpProvider2.free.sub(quoteAssetFundsLpProvider2BeforeRemoval.free))
            .sub(new BN((100_000_000_000).toString()))
        ).to.be.bignumber.closeTo(
          new BN(amountEarnedFeesQuoteAssetWallet1.toString()).add(new BN(priceFirstTrade.toFixed(0))),
          new BN(1)
        );
      }
    );

    it(
      "#3.8 After adding funds, and after a trade, transferring part of them to another wallet " +
        "the initial wallet can withdraw the provided liquidity with received fees, " +
        "while the second wallet won't receive fees",
      async function () {
        this.timeout(5 * 60 * 1000);
        /*
          1. Provide liquidity
          2. Trade
          3. Move 50% of liquidity to another wallet.
          4. Remove liquidity of first wallet (expect fees)
          5. Remove liquidity of second wallet (expect no fees)
        */

        // 1. Providing liquidity
        const lpAssetAmounts = api.createType("BTreeMap<u128, u128>", {
          "4": Pica(100_000),
          "130": USDT(100_000)
        });
        await sendAndWaitForSuccess(
          api,
          walletLpProvider1,
          api.events.pablo.LiquidityAdded.is,
          api.tx.pablo.addLiquidity(hardCodedPool1.poolId, lpAssetAmounts, 0, true)
        );

        // 2. Trade
        const transferAmount = Pica(100);
        // Getting funds in pool before tx
        const baseAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.baseAssetId
        );
        const quoteAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.quoteAssetId
        );
        // Getting user funds before tx
        const baseAssetTraderFundsBeforeTx = await api.query.tokens.accounts(
          walletTrader1.publicKey,
          hardCodedPool1.baseAssetId
        );
        const quoteAssetTraderFundsBeforeTx = await api.query.tokens.accounts(
          walletTrader1.publicKey,
          hardCodedPool1.quoteAssetId
        );
        const Bo = BigNumber(baseAssetFundsCurrentlyInPoolsBeforeTx.free.toString());
        const Bi = BigNumber(quoteAssetFundsCurrentlyInPoolsBeforeTx.free.toString());
        const priceFirstTrade = calculateInGivenOut(
          Bo,
          Bi,
          BigNumber(transferAmount.toString()),
          BigNumber(5),
          BigNumber(5)
        );
        const txResult = await sendAndWaitForSuccess(
          api,
          walletTrader1,
          api.events.pablo.Swapped.is,
          api.tx.pablo.buy(hardCodedPool1.poolId, 130, { assetId: 4, amount: transferAmount }, false)
        );
        const amountEarnedFeesQuoteAssetWallet1 = BigNumber(txResult.data[6].fee.toString());
        await verifyBuySwapOperation(
          api,
          transferAmount,
          walletTrader1,
          txResult,
          baseAssetFundsCurrentlyInPoolsBeforeTx,
          quoteAssetFundsCurrentlyInPoolsBeforeTx,
          baseAssetTraderFundsBeforeTx,
          quoteAssetTraderFundsBeforeTx,
          "buy",
          hardCodedPool1
        );
        // 3. Transferring LP tokens
        const receivingWallet = walletLpProvider2.publicKey;
        const lpAmount = <OrmlTokensAccountData>(
          await api.query.tokens.accounts(walletLpProvider1.publicKey, hardCodedPool1.lpTokenId)
        );
        const amountToTransfer = lpAmount.free.muln(0.5);
        const {
          data: [resultCurrencyId, resultFrom, resultTo, resultAmount]
        } = await sendAndWaitForSuccess(
          api,
          walletLpProvider1,
          api.events.tokens.Transfer.is,
          api.tx.assets.transfer(hardCodedPool1.lpTokenId, receivingWallet, amountToTransfer, false)
        );

        const quoteAssetFundsLpProvider1BeforeRemoval = await api.query.tokens.accounts(
          walletLpProvider1.publicKey,
          hardCodedPool1.quoteAssetId
        );
        const quoteAssetFundsLpProvider2BeforeRemoval = await api.query.tokens.accounts(
          walletLpProvider2.publicKey,
          hardCodedPool1.quoteAssetId
        );
        // 4. Remove initial liquidity
        const lpTokenId = hardCodedPool1.lpTokenId;
        const lpTokenAccountInfo1 = await api.query.tokens.accounts(walletLpProvider1.publicKey, lpTokenId);
        const availableLpTokens1 = lpTokenAccountInfo1.free;

        const removeAssetsMinAmounts = api.createType("BTreeMap<u128, u128>", {
          "4": Pica(0),
          "130": USDT(0)
        });

        const {
          data: [resultWho, resultPoolId, resultReturnedAssetsAmount, resultMintedLp]
        } = await sendAndWaitForSuccess(
          api,
          walletLpProvider1,
          api.events.pablo.LiquidityRemoved.is,
          // Parameters: poolId, assetsMap, minMintAmount, keepAlive
          api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, availableLpTokens1, removeAssetsMinAmounts) // ToDo: Update pool id w/ created pool!
        );

        // 5. Remove initial liquidity
        const lpTokenAccountInfo2 = await api.query.tokens.accounts(walletLpProvider2.publicKey, lpTokenId);
        const availableLpTokens2 = lpTokenAccountInfo2.free;

        await sendAndWaitForSuccess(
          api,
          walletLpProvider2,
          api.events.pablo.LiquidityRemoved.is,
          // Parameters: poolId, assetsMap, minMintAmount, keepAlive
          api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, availableLpTokens2, removeAssetsMinAmounts) // ToDo: Update pool id w/ created pool!
        );

        const finalQuoteAssetFundsLpProvider1 = await api.query.tokens.accounts(
          walletLpProvider1.publicKey,
          hardCodedPool1.quoteAssetId
        );
        const finalQuoteAssetFundsLpProvider2 = await api.query.tokens.accounts(
          walletLpProvider2.publicKey,
          hardCodedPool1.quoteAssetId
        );
        expect(
          finalQuoteAssetFundsLpProvider1.free
            .sub(quoteAssetFundsLpProvider1BeforeRemoval.free)
            .add(finalQuoteAssetFundsLpProvider2.free.sub(quoteAssetFundsLpProvider2BeforeRemoval.free))
            .sub(new BN((100_000_000_000).toString()))
        ).to.be.bignumber.closeTo(
          new BN(amountEarnedFeesQuoteAssetWallet1.toString()).add(new BN(priceFirstTrade.toFixed(0))),
          new BN(1)
        );
      }
    );

    it(
      "#3.9 After adding funds, and after a trade, another wallet can add liquidity, withdraw it again without " +
        "fees, and the first wallet can withdraw its funds with fees",
      async function () {
        this.timeout(5 * 60 * 1000);
        /*
          1. Provide liquidity using first wallet
          2. Trade
          3. Provide liquidity using second wallet
          4. Remove liquidity of second wallet (expect no fees)
          5. Remove liquidity of first wallet (expect fees)
        */

        /**
         * 1. Providing liquidity
         */
        const lpAssetAmounts = api.createType("BTreeMap<u128, u128>", {
          "4": Pica(100_000),
          "130": USDT(10_000)
        });
        await sendAndWaitForSuccess(
          api,
          walletLpProvider1,
          api.events.pablo.LiquidityAdded.is,
          api.tx.pablo.addLiquidity(hardCodedPool1.poolId, lpAssetAmounts, 0, true)
        );

        /**
         * 2. Trade
         */

        const transferAmount = Pica(1);
        // Getting funds in pool before tx
        const baseAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.baseAssetId
        );
        const quoteAssetFundsCurrentlyInPoolsBeforeTx = await api.query.tokens.accounts(
          hardCodedPool1.poolWalletAddress,
          hardCodedPool1.quoteAssetId
        );
        // Getting user funds before tx
        const baseAssetTraderFundsBeforeTx = await api.query.tokens.accounts(
          walletTrader1.publicKey,
          hardCodedPool1.baseAssetId
        );
        const quoteAssetTraderFundsBeforeTx = await api.query.tokens.accounts(
          walletTrader1.publicKey,
          hardCodedPool1.quoteAssetId
        );

        const Bo = BigNumber(baseAssetFundsCurrentlyInPoolsBeforeTx.free.toString());
        const Bi = BigNumber(quoteAssetFundsCurrentlyInPoolsBeforeTx.free.toString());
        const priceFirstTrade = calculateInGivenOut(
          Bo,
          Bi,
          BigNumber(transferAmount.toString()),
          BigNumber(5),
          BigNumber(5)
        );

        // Trade
        const txResult = await sendAndWaitForSuccess(
          api,
          walletTrader1,
          api.events.pablo.Swapped.is,
          api.tx.pablo.buy(hardCodedPool1.poolId, 130, { assetId: 4, amount: transferAmount }, false)
        );
        await verifyBuySwapOperation(
          api,
          transferAmount,
          walletTrader1,
          txResult,
          baseAssetFundsCurrentlyInPoolsBeforeTx,
          quoteAssetFundsCurrentlyInPoolsBeforeTx,
          baseAssetTraderFundsBeforeTx,
          quoteAssetTraderFundsBeforeTx,
          "buy",
          hardCodedPool1
        );
        const amountEarnedFeesQuoteAssetWallet1 = BigNumber(txResult.data[6].fee.toString());

        /**
         * 3. Provide liquidity using second wallet
         */
        await sendAndWaitForSuccess(
          api,
          walletLpProvider2,
          api.events.pablo.LiquidityAdded.is,
          api.tx.pablo.addLiquidity(hardCodedPool1.poolId, lpAssetAmounts, 0, true)
        );

        /**
         * 4. Remove initial liquidity
         */
        const quoteAssetFundsLpProvider1BeforeRemoval = await api.query.tokens.accounts(
          walletLpProvider1.publicKey,
          hardCodedPool1.quoteAssetId
        );

        const lpTokenId = hardCodedPool1.lpTokenId;
        const lpTokenAccountInfo1 = await api.query.tokens.accounts(walletLpProvider1.publicKey, lpTokenId);
        const availableLpTokens1 = lpTokenAccountInfo1.free;

        const removeAssetsMinAmounts = api.createType("BTreeMap<u128, u128>", {
          "4": Pica(0),
          "130": USDT(0)
        });

        await sendAndWaitForSuccess(
          api,
          walletLpProvider1,
          api.events.pablo.LiquidityRemoved.is,
          // Parameters: poolId, assetsMap, minMintAmount, keepAlive
          api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, availableLpTokens1, removeAssetsMinAmounts) // ToDo: Update pool id w/ created pool!
        );

        /**
         * 5. Remove initial liquidity
         */
        const quoteAssetFundsLpProvider2BeforeRemoval = await api.query.tokens.accounts(
          walletLpProvider2.publicKey,
          hardCodedPool1.quoteAssetId
        );

        const lpTokenAccountInfo2 = await api.query.tokens.accounts(walletLpProvider2.publicKey, lpTokenId);
        const availableLpTokens2 = lpTokenAccountInfo2.free;

        await sendAndWaitForSuccess(
          api,
          walletLpProvider2,
          api.events.pablo.LiquidityRemoved.is,
          // Parameters: poolId, assetsMap, minMintAmount, keepAlive
          api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, availableLpTokens2, removeAssetsMinAmounts) // ToDo: Update pool id w/ created pool!
        );

        // Make sure lpProvider2 receives fewer LP Tokens than #1.

        const finalQuoteAssetFundsLpProvider1 = await api.query.tokens.accounts(
          walletLpProvider1.publicKey,
          hardCodedPool1.quoteAssetId
        );
        const finalQuoteAssetFundsLpProvider2 = await api.query.tokens.accounts(
          walletLpProvider2.publicKey,
          hardCodedPool1.quoteAssetId
        );
        const lpRatioFactorBetweenWallets = BigNumber(lpTokenAccountInfo1.free.toString()).dividedBy(
          BigNumber(lpTokenAccountInfo2.free.toString())
        );

        const returnedQuoteAssetFundsWallet1 = finalQuoteAssetFundsLpProvider1.free.sub(
          quoteAssetFundsLpProvider1BeforeRemoval.free
        );
        const returnedQuoteAssetFundsWallet2 = finalQuoteAssetFundsLpProvider2.free.sub(
          quoteAssetFundsLpProvider2BeforeRemoval.free
        );

        const receivedQuoteAssetFundsRatioFactorBetweenWallets = BigNumber(
          returnedQuoteAssetFundsWallet1.toString()
        ).dividedBy(BigNumber(returnedQuoteAssetFundsWallet2.toString()));

        expect(availableLpTokens1).to.be.bignumber.greaterThan(availableLpTokens2);

        // // ToDo (D. Roth): Fix rounding issue!
        // expect(Number(lpRatioFactorBetweenWallets.toFixed(6))).to.be.equal(
        //   Number(receivedQuoteAssetFundsRatioFactorBetweenWallets.toFixed(6))
        // );
        // expect(returnedQuoteAssetFundsWallet1).to.be.bignumber.greaterThan(returnedQuoteAssetFundsWallet2);
        // expect(
        //   returnedQuoteAssetFundsWallet1.add(returnedQuoteAssetFundsWallet2).sub(new BN(USDT(200_000).toString()))
        // ).to.be.bignumber.equal(
        //   new BN(amountEarnedFeesQuoteAssetWallet1.toString()).add(new BN(priceFirstTrade.toFixed(0)))
        // );

        expect(
          finalQuoteAssetFundsLpProvider1.free
            .sub(quoteAssetFundsLpProvider1BeforeRemoval.free)
            .add(finalQuoteAssetFundsLpProvider2.free.sub(quoteAssetFundsLpProvider2BeforeRemoval.free))
            .sub(new BN((20_000_000_000).toString()))
        ).to.be.bignumber.closeTo(
          new BN(amountEarnedFeesQuoteAssetWallet1.toString()).add(new BN(priceFirstTrade.toFixed(0))),
          new BN(1)
        );
      }
    );
  });

  describe("2. Providing liquidity pt. 3", function () {
    it("#2.3  I can not add only the base or quote asset as liquidity", async function () {
      // First, adding some liquidity, since the pool is empty at this point.
      const ksmAmount = Pica(10_000);
      const usdtAmount = USDT(100_000);
      const assetsInitialLiquidity = api.createType("BTreeMap<u128, u128>", {
        "4": ksmAmount,
        "130": usdtAmount
      });
      await sendAndWaitForSuccess(
        api,
        walletLpProvider1,
        api.events.pablo.LiquidityAdded.is,
        api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assetsInitialLiquidity, 0, true)
      );

      // Actual Test
      const assets = api.createType("BTreeMap<u128, u128>", {
        "4": Pica(10_000)
      });
      // 1. Adding one-sided liquidity
      await sendAndWaitForSuccess(
        api,
        walletLpProvider3,
        api.events.pablo.LiquidityAdded.is,
        api.tx.pablo.addLiquidity(hardCodedPool1.poolId, assets, 0, true)
      );

      const baseAssetFundsTraderBefore = await api.query.tokens.accounts(
        walletLpProvider3.publicKey,
        hardCodedPool1.baseAssetId
      );
      const quoteAssetFundsTraderBefore = await api.query.tokens.accounts(
        walletLpProvider3.publicKey,
        hardCodedPool1.quoteAssetId
      );
      const lpTokenAccountInfo = await api.query.tokens.accounts(walletLpProvider3.publicKey, hardCodedPool1.lpTokenId);
      const availableLpTokens = lpTokenAccountInfo.free;

      // Removing added liquidity again, will remove the KSM as well as the USDT amount.
      const assetsRemovalMinReceive = api.createType("BTreeMap<u128, u128>", {
        "4": 0,
        "130": 0
      });
      const {
        data: [resultWhoRemoved, resultPoolIdRemoved, resultAssetsAmountsRemoved]
      } = await sendAndWaitForSuccess(
        api,
        walletLpProvider3,
        api.events.pablo.LiquidityRemoved.is,
        api.tx.pablo.removeLiquidity(hardCodedPool1.poolId, availableLpTokens, assetsRemovalMinReceive)
      );
      expect(resultWhoRemoved.toString()).to.be.equal(
        api.createType("AccountId32", walletLpProvider3.publicKey).toString()
      );
      expect(resultPoolIdRemoved).to.be.bignumber.equal(new BN(hardCodedPool1.poolId.toString()));
      const baseAssetFundsTraderAfterRemoval = await api.query.tokens.accounts(
        walletLpProvider3.publicKey,
        hardCodedPool1.baseAssetId
      );
      const quoteAssetFundsTraderAfterRemoval = await api.query.tokens.accounts(
        walletLpProvider3.publicKey,
        hardCodedPool1.quoteAssetId
      );
      expect(quoteAssetFundsTraderAfterRemoval.free).to.be.bignumber.greaterThan(baseAssetFundsTraderAfterRemoval.free);
      expect(baseAssetFundsTraderAfterRemoval.free).to.be.bignumber.equal(
        // @ts-ignore
        baseAssetFundsTraderBefore.free.add(new BN(resultAssetsAmountsRemoved.toPrimitive()[4].toString()))
      );
      expect(quoteAssetFundsTraderAfterRemoval.free).to.be.bignumber.equal(
        // @ts-ignore
        quoteAssetFundsTraderBefore.free.add(new BN(resultAssetsAmountsRemoved.toPrimitive()[130].toString()))
      );
    });
  });
});
