"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const connectionHelper_1 = require("@composable/utils/connectionHelper");
const mintingHelper_1 = require("@composable/utils/mintingHelper");
const polkadotjs_1 = require("@composable/utils/polkadotjs");
const walletHelper_1 = require("@composable/utils/walletHelper");
const chai_1 = require("chai");
const pabloTestHelper_1 = require("../pablo/testHandlers/pabloTestHelper");
/**
 * Single Test to check the fix for hallborn audit fix.
 * This test will create two constant product pools with various assets and check if the audit fix is working.
 * The description of the issue:
 * Inside the uniswap and curv-amm pallets, the create function calls
 * do_create_pool without restrictions, allowing anyone to create a pool
 * of arbitrary pairs, which leads to a price manipulation risk.
 * dex-router pallet auditing where theupdate_route allows the caller to
 * create, update or delete existing routers, the function was found to
 * lack implementing a custom origin to restrict access to this function.
 * The test validates the fix for the audit issue and confirms that now updating route is permissioned.
 */
describe("DexRouter Tests", function () {
    let api;
    let poolId, poolId2;
    let eth, usdt, usdc, dai;
    let badAsset;
    let walletId1, walletId2, sudoKey;
    let fee, baseWeight;
    this.timeout(2 * 60 * 1000);
    before("Initialize variables", async function () {
        const { newClient, newKeyring } = await (0, connectionHelper_1.getNewConnection)();
        api = newClient;
        const { devWalletAlice, devWalletEve, devWalletFerdie } = (0, walletHelper_1.getDevWallets)(newKeyring);
        sudoKey = devWalletAlice;
        walletId1 = devWalletEve.derive("/test/constantProductDex/walletId1");
        walletId2 = devWalletFerdie.derive("/test/constantProductDex/walletId2");
        eth = 5;
        usdt = 6;
        usdc = 7;
        dai = 9;
        badAsset = 51;
        //sets the fee to 1.00%/Type Permill
        fee = 10000;
        //sets the owner fee to 5.00%/Type Permill
        baseWeight = 50000;
    });
    before("Minting assets", async function () {
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletId1, sudoKey, [1, eth, usdc, usdt, dai]);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletId2, sudoKey, [1, eth, usdc, usdt, dai]);
    });
    after("Closing the connection", async function () {
        await api.disconnect();
    });
    it("Hallborn Fix Validation", async function () {
        this.timeout(5 * 60 * 1000);
        poolId = await (0, pabloTestHelper_1.createConsProdPool)(api, walletId1, walletId1, eth, usdc, fee, baseWeight);
        poolId2 = await (0, pabloTestHelper_1.createConsProdPool)(api, walletId1, walletId1, usdt, eth, fee, baseWeight);
        const assetPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
            base: usdt,
            quote: usdc
        });
        const route = api.createType("Vec<u128>", [api.createType("u128", poolId), api.createType("u128", poolId2)]);
        await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.dexRouter.updateRoute(assetPair, route)));
        const badPool1 = await (0, pabloTestHelper_1.createConsProdPool)(api, walletId2, walletId2, badAsset, usdc, fee, baseWeight);
        const badPool2 = await (0, pabloTestHelper_1.createConsProdPool)(api, walletId2, walletId2, usdt, badAsset, fee, baseWeight);
        const badRoute = api.createType("Vec<u128>", [api.createType("u128", badPool1), api.createType("u128", badPool2)]);
        await (0, polkadotjs_1.sendAndWaitForSuccess)(api, walletId2, api.events.dexRouter.RouteUpdated.is, api.tx.dexRouter.updateRoute(assetPair, badRoute)
        //Verify that the update route is permissioned
        ).catch(error => (0, chai_1.expect)(error.message).to.contain("BadOrigin"));
    });
});
//# sourceMappingURL=hal07-Test.js.map