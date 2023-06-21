import { getNewConnection } from "@composable/utils/connectionHelper";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { getDevWallets } from "@composable/utils/walletHelper";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { expect } from "chai";
import { createConsProdPool } from "../dexRouter/testHandlers/dexRouterHelper";
import BN from "bn.js";

// DEX router pallet integration test

// In these tests we are testing the following extrinsics:
//  - updateRoute
//  - addLiquidity
//  - removeLiquidity
//  - buy
//  - exchange
//  - sell

describe("DexRouterPallet Tests", function () {
  let api: ApiPromise;
  let eth: number, usdt: number, usdc: number, dai: number;
  let walletId1: KeyringPair, walletId2: KeyringPair, sudoKey: KeyringPair;
  let fee: number, baseWeight: number;
  let poolId1: number, poolId2: number, poolId3: number;
  this.timeout(2 * 60 * 1000);

  before("Initialize variables", async function () {
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice, devWalletEve, devWalletFerdie } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    walletId1 = devWalletEve.derive("/test/constantProductDex/walletId1");
    walletId2 = devWalletFerdie.derive("/test/constantProductDex/walletId2");
    eth = 5;
    usdt = 6;
    usdc = 7;
    dai = 9;
    //sets the fee to 1.00%/Type Permill
    fee = 10000;
    baseWeight = 500000;
  });

  before("Minting assets", async function () {
    await mintAssetsToWallet(api, walletId1, sudoKey, [1, eth, usdc, usdt, dai]);
    await mintAssetsToWallet(api, walletId2, sudoKey, [1, eth, usdc, usdt, dai]);
  });

  before("Creating pools", async function () {
    poolId1 = await createConsProdPool(api, sudoKey, walletId1, usdt, eth, fee, baseWeight);
    expect(poolId1).to.not.be.an("Error");
    poolId2 = await createConsProdPool(api, sudoKey, walletId1, usdc, usdt, fee, baseWeight);
    expect(poolId2).to.not.be.an("Error");
    poolId3 = await createConsProdPool(api, sudoKey, walletId1, dai, usdc, fee, baseWeight);
    expect(poolId3).to.not.be.an("Error");
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  it("Create route #1 for pablo pools", async function () {
    this.timeout(2 * 60 * 1000);
    // create route for pool 1 (USDT-ETH)
    const assetPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: usdt,
      quote: eth
    });
    const route = api.createType("Vec<u128>", [api.createType("u128", poolId1)]);
    const {
      data: [result]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.dexRouter.updateRoute(assetPair, route))
    );
    expect(result.isOk).to.be.true;
  });

  it("Create route #2 for pablo pools", async function () {
    this.timeout(2 * 60 * 1000);
    // create route for pool 2 (USDC-USDT)
    const assetPair2 = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: usdc,
      quote: usdt
    });
    const route2 = api.createType("Vec<u128>", [api.createType("u128", poolId2)]);
    const {
      data: [result2]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.dexRouter.updateRoute(assetPair2, route2))
    );
    expect(result2.isOk).to.be.true;
  });

  it("Create route #3 for pablo pools", async function () {
    this.timeout(2 * 60 * 1000);
    // create route for pool 3 (DAI-USDC)
    const assetPair3 = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: dai,
      quote: usdc
    });
    const route3 = api.createType("Vec<u128>", [api.createType("u128", poolId3)]);
    const {
      data: [result3]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.dexRouter.updateRoute(assetPair3, route3))
    );
    expect(result3.isOk).to.be.true;
  });

  it("Create route #4 for pablo pools", async function () {
    this.timeout(2 * 60 * 1000);
    // create route for USDC-ETH pair (pool 1 <--> pool 2)
    const assetPair4 = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: usdc,
      quote: eth
    });
    const route4 = api.createType("Vec<u128>", [api.createType("u128", poolId1), api.createType("u128", poolId2)]);
    const {
      data: [result4]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.dexRouter.updateRoute(assetPair4, route4))
    );
    expect(result4.isOk).to.be.true;
  });

  it("Create route #5 for pablo pools", async function () {
    this.timeout(2 * 60 * 1000);
    // create route for DAI-USDT pair (pool 2 <--> pool3)
    const assetPair5 = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: dai,
      quote: usdt
    });
    const route5 = api.createType("Vec<u128>", [api.createType("u128", poolId2), api.createType("u128", poolId3)]);
    const {
      data: [result5]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.dexRouter.updateRoute(assetPair5, route5))
    );
    expect(result5.isOk).to.be.true;
  });

  it("Create route #6 for pablo pools", async function () {
    this.timeout(2 * 60 * 1000);
    // create route for DAI-ETH pair (pool 1 <--> pool 2 <--> pool3)
    const assetPair6 = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: dai,
      quote: eth
    });
    const route6 = api.createType("Vec<u128>", [
      api.createType("u128", poolId1),
      api.createType("u128", poolId2),
      api.createType("u128", poolId3)
    ]);
    const {
      data: [result6]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.dexRouter.updateRoute(assetPair6, route6))
    );
    expect(result6.isOk).to.be.true;
  });

  it("Add liquidity to pablo pool (USDT-ETH)", async function () {
    this.timeout(5 * 60 * 1000);
    const USDTAmount = 1000000000000000;
    const ETHAmount = 1000000000000000;
    const minimumMint = 0;
    //set tx parameters
    const assetPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: usdt,
      quote: eth
    });
    const baseAmount = api.createType("u128", USDTAmount);
    const quoteAmount = api.createType("u128", ETHAmount);
    const minMintAmount = api.createType("u128", minimumMint);
    const keepAlive = api.createType("bool", false);
    // extrinsic call
    // ToDo (D. Roth): Re- enable after pablo rework!
    /*const {
      data: [, , baseAmountInTransfer, quoteAmountInTransfer, mintedLp]
    } = await sendAndWaitForSuccess(
      api,
      walletId2,
      api.events.pablo.LiquidityAdded.is,
      api.tx.dexRouter.addLiquidity(assetPair, baseAmount, quoteAmount, minMintAmount, keepAlive)
    );
    // Assertions
    expect(baseAmountInTransfer.toString()).to.be.equal(baseAmount.toString());
    expect(quoteAmountInTransfer.toString()).to.be.equal(quoteAmount.toString());
    expect(new BN(mintedLp).gt(new BN(minimumMint))).to.be.true;*/
  });

  it("Add liquidity to pablo pool (USDC-USDT)", async function () {
    this.timeout(5 * 60 * 1000);
    const USDCAmount = 1000000000000000;
    const USDTAmount = 1000000000000000;
    const minimumMint = 0;
    //set tx parameters
    const assetPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: usdc,
      quote: usdt
    });
    const baseAmount = api.createType("u128", USDCAmount);
    const quoteAmount = api.createType("u128", USDTAmount);
    const minMintAmount = api.createType("u128", minimumMint);
    const keepAlive = api.createType("bool", false);
    // ToDo (D. Roth): Re- enable after pablo rework!
    //extrinsic call
    /*
    const {
      data: [, , baseAmountInTransfer, quoteAmountInTransfer, mintedLp]
    } = await sendAndWaitForSuccess(
      api,
      walletId2,
      api.events.pablo.LiquidityAdded.is,
      api.tx.dexRouter.addLiquidity(assetPair, baseAmount, quoteAmount, minMintAmount, keepAlive)
    );
    //Assertions
    expect(baseAmountInTransfer.toString()).to.be.equal(baseAmount.toString());
    expect(quoteAmountInTransfer.toString()).to.be.equal(quoteAmount.toString());
    expect(new BN(mintedLp).gt(new BN(minimumMint))).to.be.true;*/
  });

  it("Add liquidity to pablo pool (DAI-USDC)", async function () {
    this.timeout(5 * 60 * 1000);
    const DAIAmount = 1000000000000000;
    const USDCAmount = 1000000000000000;
    const minimumMint = 0;
    //set tx parameters
    const assetPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: dai,
      quote: usdc
    });
    const baseAmount = api.createType("u128", DAIAmount);
    const quoteAmount = api.createType("u128", USDCAmount);
    const minMintAmount = api.createType("u128", minimumMint);
    const keepAlive = api.createType("bool", false);
    // ToDo (D. Roth): Re- enable after pablo rework!
    //extrinsic call
    /*
    const {
      data: [, , baseAmountInTransfer, quoteAmountInTransfer, mintedLp]
    } = await sendAndWaitForSuccess(
      api,
      walletId2,
      api.events.pablo.LiquidityAdded.is,
      api.tx.dexRouter.addLiquidity(assetPair, baseAmount, quoteAmount, minMintAmount, keepAlive)
    );
    //Assertions
    expect(baseAmountInTransfer.toString()).to.be.equal(baseAmount.toString());
    expect(quoteAmountInTransfer.toString()).to.be.equal(quoteAmount.toString());
    expect(new BN(mintedLp).gt(new BN(minimumMint))).to.be.true;*/
  });

  it("update route (USDC-USDT)", async function () {
    // Create new pool for USDC-USDT
    const newPoolId = await createConsProdPool(api, sudoKey, walletId1, usdc, usdt, fee, baseWeight);
    // update route for pool 2 USDC-USDT
    const assetPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: usdc,
      quote: usdt
    });
    const route = api.createType("Vec<u128>", [api.createType("u128", newPoolId)]);
    const {
      data: [baseTokenId, quoteTokenId, , newRoute]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.dexRouter.RouteUpdated.is,
      api.tx.sudo.sudo(api.tx.dexRouter.updateRoute(assetPair, route))
    );
    // Add liquidity to USDC-USDT pool so next test doesn't fail
    const USDCAmount = 1000000000000000;
    const USDTAmount = 1000000000000000;
    const minimumMint = 0;
    const baseAmount = api.createType("u128", USDCAmount);
    const quoteAmount = api.createType("u128", USDTAmount);
    const minMintAmount = api.createType("u128", minimumMint);
    const keepAlive = api.createType("bool", false);
    // ToDo (D. Roth): Re- enable after pablo rework!
    /*
    await sendAndWaitForSuccess(
      api,
      walletId2,
      api.events.pablo.LiquidityAdded.is,
      api.tx.dexRouter.addLiquidity(assetPair, baseAmount, quoteAmount, minMintAmount, keepAlive)
    );
    // Assertions
    expect(baseTokenId.toString()).eq(usdc.toString());
    expect(quoteTokenId.toString()).eq(usdt.toString());
    expect(newRoute[0].toString()).eq(newPoolId.toString());*/
  });

  it("Remove liquidity from pablo pool (USDC-USDT)", async function () {
    this.timeout(5 * 60 * 1000);
    const assetAmount = 10000000000000;
    const minUSDCAmount = 1000000000000;
    const minUSDTAmount = 1000000000000;
    //set tx parameters\
    const assetPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: usdc,
      quote: usdt
    });
    const lpAmount = api.createType("u128", assetAmount);
    const minBaseAmount = api.createType("u128", minUSDCAmount);
    const minQuoteAmount = api.createType("u128", minUSDTAmount);
    // ToDo (D. Roth): Re- enable after pablo rework!
    //extrinsic call
    /*
    const {
      data: [, , baseAmountInTransfer, quoteAmountInTransfer]
    } = await sendAndWaitForSuccess(
      api,
      walletId2,
      api.events.pablo.LiquidityRemoved.is,
      api.tx.dexRouter.removeLiquidity(assetPair, lpAmount, minBaseAmount, minQuoteAmount)
    );
    //Assertions
    expect(new BN(baseAmountInTransfer.toString()).gt(new BN(minUSDCAmount.toString()))).to.be.true;
    expect(new BN(quoteAmountInTransfer.toString()).gt(new BN(minUSDCAmount.toString()))).to.be.true;*/
  });

  it("Buy ETH via route found in router (1 hop)", async function () {
    this.timeout(5 * 60 * 1000);
    //get initial data
    const initialETHbalance = new BN((await api.rpc.assets.balanceOf(eth.toString(), walletId2.publicKey)).toString());
    const initialUSDCbalance = new BN(
      (await api.rpc.assets.balanceOf(usdc.toString(), walletId2.publicKey)).toString()
    );
    //set tx parameters
    const ETHAmount = 1000000000000;
    const assetPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: eth,
      quote: usdc
    });
    const amount = api.createType("u128", ETHAmount);
    const minReceive = api.createType("u128", 0);
    // ToDo (D. Roth): Re- enable after pablo rework!
    //extrinsic call
    /*
    await sendAndWaitForSuccess(
      api,
      walletId2,
      api.events.pablo.Swapped.is, // verify
      api.tx.dexRouter.buy(assetPair, amount, minReceive)
    );
    //get final data
    const finalETHbalance = new BN((await api.rpc.assets.balanceOf(eth.toString(), walletId2.publicKey)).toString());
    const finalUSDCbalance = new BN((await api.rpc.assets.balanceOf(usdc.toString(), walletId2.publicKey)).toString());
    //Assertions
    expect(initialETHbalance.lt(finalETHbalance)).to.be.true;
    expect(initialUSDCbalance.gt(finalUSDCbalance)).to.be.true;*/
  });

  it("Buy ETH via route found in router (2 hops)", async function () {
    this.timeout(5 * 60 * 1000);
    //get initial data
    const initialETHbalance = new BN((await api.rpc.assets.balanceOf(eth.toString(), walletId2.publicKey)).toString());
    const initialDAIbalance = new BN((await api.rpc.assets.balanceOf(usdc.toString(), walletId2.publicKey)).toString());
    //set tx parameters
    const ETHAmount = 1000000000000;
    const assetPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: eth,
      quote: dai
    });
    const amount = api.createType("u128", ETHAmount);
    const minReceive = api.createType("u128", 0);
    //extrinsic call
    // ToDo (D. Roth): Re- enable after pablo rework!
    /*
    await sendAndWaitForSuccess(
      api,
      walletId2,
      api.events.pablo.Swapped.is, // verify
      api.tx.dexRouter.buy(assetPair, amount, minReceive)
    );
    //get final data
    const finalETHbalance = new BN((await api.rpc.assets.balanceOf(eth.toString(), walletId2.publicKey)).toString());
    const finalDAIbalance = new BN((await api.rpc.assets.balanceOf(dai.toString(), walletId2.publicKey)).toString());
    //Assertions
    expect(initialETHbalance).to.be.bignumber.lessThan(finalETHbalance);
    expect(initialDAIbalance).to.be.bignumber.lessThan(finalDAIbalance);*/
  });

  it("Exchange ETH for USDC via route found in router (1 hop)", async function () {
    this.timeout(5 * 60 * 1000);
    //get initial data
    const initialETHbalance = new BN((await api.rpc.assets.balanceOf(eth.toString(), walletId2.publicKey)).toString());
    const initialUSDCbalance = new BN(
      (await api.rpc.assets.balanceOf(usdc.toString(), walletId2.publicKey)).toString()
    );
    //set tx parameters
    const ETHAmount = 1000;
    const assetPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: eth,
      quote: usdc
    });
    const amount = api.createType("u128", ETHAmount);
    const minReceive = api.createType("u128", 0);
    //extrinsic call
    await sendAndWaitForSuccess(
      api,
      walletId2,
      api.events.pablo.Swapped.is, // verify
      api.tx.dexRouter.exchange(assetPair, amount, minReceive)
    );
    //get final data
    const finalETHbalance = new BN((await api.rpc.assets.balanceOf(eth.toString(), walletId2.publicKey)).toString());
    const finalUSDCbalance = new BN((await api.rpc.assets.balanceOf(usdc.toString(), walletId2.publicKey)).toString());
    //Assertions
    expect(initialETHbalance.lt(finalETHbalance)).to.be.true;
    expect(initialUSDCbalance.gt(finalUSDCbalance)).to.be.true;
  });

  it("Sell ETH via route found in router (1 hop)", async function () {
    this.timeout(5 * 60 * 1000);
    //get initial data
    const initialETHbalance = new BN((await api.rpc.assets.balanceOf(eth.toString(), walletId2.publicKey)).toString());
    const initialUSDCbalance = new BN(
      (await api.rpc.assets.balanceOf(usdc.toString(), walletId2.publicKey)).toString()
    );
    //set tx parameters
    const ETHAmount = 100_000_000_000;
    const assetPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: usdc,
      quote: eth
    });
    const amount = api.createType("u128", ETHAmount);
    const minReceive = api.createType("u128", 0);
    //extrinsic call
    await sendAndWaitForSuccess(
      api,
      walletId2,
      api.events.pablo.Swapped.is, // verify
      api.tx.dexRouter.sell(assetPair, amount, minReceive)
    );
    //get final data
    const finalETHbalance = new BN((await api.rpc.assets.balanceOf(eth.toString(), walletId2.publicKey)).toString());
    const finalUSDCbalance = new BN((await api.rpc.assets.balanceOf(usdc.toString(), walletId2.publicKey)).toString());
    //Assertions
    expect(initialETHbalance.gt(finalETHbalance)).to.be.true;
    expect(initialUSDCbalance.lt(finalUSDCbalance)).to.be.true;
  });
});
