import { getNewConnection } from "@composable/utils/connectionHelper";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { getDevWallets } from "@composable/utils/walletHelper";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { expect } from "chai";
import { createConsProdPool, getPoolInfo } from "../dexRouter/testHandlers/dexRouterHelper";
import BN from "bn.js";

// DEX router pallet integration test

// In these tests we are testing the following extrinsics:
//  - updateRoute
//  - addLiquidity
//  - removeLiquidity
//  - buy
//  - exchange
//  - sell
// Tests:
//  - Create route for pablo pool
//  - Add liquidity to underlying pablo pool
//  - Remove liquidity from the underlying pablo pool
//  - Buy amount of quote asset via route found in router
//  - *Exchange amount of quote asset via route found in router
//  - *Sell amount of quote asset via route found in router



describe("DexRouterPallet Tests", function () {
  let api: ApiPromise;
  let eth: number, usdt: number, usdc: number, dai: number;
  let badAsset: number;
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

  before("Creating pools", async function()  {
    poolId1 = await createConsProdPool(api, walletId1, walletId1, usdt, eth, fee, baseWeight);
    poolId2 = await createConsProdPool(api, walletId1, walletId1, usdt, usdc, fee, baseWeight);
    poolId3 = await createConsProdPool(api, walletId1, walletId1, usdc, dai, fee, baseWeight);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  it("Create route for pablo pools", async function() {
    this.timeout(5 * 60 * 1000);

    // Link (USDT-ETH) <-> (USDT-USDC)
    const assetPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: eth,
      quote: usdc
    });
    const route = api.createType("Vec<u128>", [api.createType("u128", poolId1), api.createType("u128", poolId2)]);
    await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.dexRouter.updateRoute(assetPair, route))
    ); 

    // Link (USDT-USDC) <-> (USDC-DAI)
    const assetPair2 = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: usdt,
      quote: dai
    });
    const route2 = api.createType("Vec<u128>", [api.createType("u128", poolId2), api.createType("u128", poolId3)]);
    await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.dexRouter.updateRoute(assetPair2, route2))
    ); 

    // Link (USDT-ETH) <-> (USDC-DAI)
    const assetPair3 = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: eth,
      quote: dai
    });
    const route3 = api.createType("Vec<u128>", [api.createType("u128", poolId1), api.createType("u128", poolId3)]);
    await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.dexRouter.updateRoute(assetPair3, route3))
    ); 
  });

  it("Add liquidity to underlying pablo pool (USDT-USDC)", async function() {
    this.timeout(5 * 60 * 1000);
    //get initial pool info
    const initialPool1Info = await getPoolInfo(api, "ConstantProduct", poolId1);
    //set tx parameters
    const assetPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
        base: usdt,
        quote: usdc
    });
    const baseAmount = api.createType("u128", 10000);
    const quoteAmount = api.createType("u128", 10000);
    const minMintAmount = api.createType("u128", 1000);
    const keepAlive = api.createType("bool", false);
    //extrinsic call
    await sendAndWaitForSuccess(
        api,
        walletId2,
        api.events.pablo.LiquidityAdded.is, // verify
        api.tx.dexRouter.addLiquidity(assetPair, baseAmount, quoteAmount, minMintAmount, keepAlive)
    );
    //get final pool info
    const finalPool1Info = await getPoolInfo(api, "ConstantProduct", poolId1);   
    //Asertions
    expect(initialPool1Info.weights[0].gt(finalPool1Info.weights[0])).to.be.true;
    expect(initialPool1Info.weights[1].gt(finalPool1Info.weights[1])).to.be.true;
  });

  it("Remove liquidity from the underlying pablo pool (USDT-USDC)", async function() {
    this.timeout(5 * 60 * 1000);
    //get initial pool info
    const initialPool1Info = await getPoolInfo(api, "ConstantProduct", poolId1);
    //set tx parameters
    const assetPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
        base: usdt,
        quote: usdc
    });
    const lpAmount = api.createType("u128", 100000);
    const minBaseAmount = api.createType("u128", 10000);
    const minQuoteAmount = api.createType("u128", 10000);
    //extrinsic call
    await sendAndWaitForSuccess(
      api,
      walletId2,
      api.events.pablo.LiquidityRemoved.is, // verify
      api.tx.dexRouter.removeLiquidity(assetPair, lpAmount, minBaseAmount, minQuoteAmount)
    );
    //get final pool info
    const finalPool1Info = await getPoolInfo(api, "ConstantProduct", poolId1);
    //Asertions
    expect(initialPool1Info.weights[0].lt(finalPool1Info.weights[0])).to.be.true;
    expect(initialPool1Info.weights[1].lt(finalPool1Info.weights[1])).to.be.true;
  });

  it("Buy amount of quote asset (ETH) via route found in router", async function() {
    this.timeout(5 * 60 * 1000);
    //get initial data
    const initialPool1Info = await getPoolInfo(api, "ConstantProduct", poolId1);
    const initialPool2Info = await getPoolInfo(api, "ConstantProduct", poolId2);
    const initialETHbalance = new BN(
      (await api.rpc.assets.balanceOf('5', walletId2.publicKey)).toString()
    )
    const initialUSDCbalance = new BN(
      (await api.rpc.assets.balanceOf('7', walletId2.publicKey)).toString()
    )
    //set tx parameters
    const assetPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
        base: usdc,
        quote: eth
    });
    const amount = api.createType("u128", 100000);
    const minReceive = api.createType("u128", 9000);
    //extrinsic call
    await sendAndWaitForSuccess(
        api,
        walletId2,
        api.events.pablo.Swapped.is, // verify
        api.tx.dexRouter.buy(assetPair, amount, minReceive)
    )
    //get final data
    const finalPool1Info = await getPoolInfo(api, "ConstantProduct", poolId1);
    const finalPool2Info = await getPoolInfo(api, "ConstantProduct", poolId2);
    const finalETHbalance = new BN(
      (await api.rpc.assets.balanceOf('5', walletId2.publicKey)).toString()
    )
    const finalUSDCbalance = new BN(
      (await api.rpc.assets.balanceOf('7', walletId2.publicKey)).toString()
    )
    //compare balances on wallet and pool
    expect(initialPool1Info.weights[0].gt(finalPool1Info.weights[0])).to.be.true;
    expect(initialPool2Info.weights[0].gt(finalPool2Info.weights[0])).to.be.true;
    expect(initialETHbalance.lt(finalETHbalance)).to.be.true;
    expect(initialUSDCbalance.gt(finalUSDCbalance)).to.be.true;
  });

  it("Exchange amount of quote asset via route found in router", async function() {
    this.timeout(5 * 60 * 1000);

  });

  it("Sell amount of quote asset via route found in router", async function() {
    this.timeout(5 * 60 * 1000);

  });

});
