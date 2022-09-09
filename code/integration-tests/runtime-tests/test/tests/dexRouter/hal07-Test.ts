import { getNewConnection } from "@composable/utils/connectionHelper";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { getDevWallets } from "@composable/utils/walletHelper";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { expect } from "chai";
import { createConsProdPool } from "../pablo/testHandlers/pabloTestHelper";
/**
 * Single Test to check the fix for halborn audit fix.
 * This test will create two constant product pools with various assets and check if the audit fix is working.
 * The description of the issue:
 * Inside the uniswap and curve-amm pallets, the create function calls
 * do_create_pool without restrictions, allowing anyone to create a pool
 * of arbitrary pairs, which leads to a price manipulation risk.
 * dex-router pallet auditing where the update_route allows the caller to
 * create, update or delete existing routers, the function was found to
 * lack implementing a custom origin to restrict access to this function.
 * The test validates the fix for the audit issue and confirms that now updating route is permissioned.
 */

describe("DexRouter Tests", function () {
  let api: ApiPromise;
  let poolId: number, poolId2: number;
  let eth: number, usdt: number, usdc: number, dai: number;
  let badAsset: number;
  let walletId1: KeyringPair, walletId2: KeyringPair, sudoKey: KeyringPair;
  let fee: number, baseWeight: number;
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
    badAsset = 51;
    //sets the fee to 1.00%/Type Permill
    fee = 10000;
    //sets the owner fee to 5.00%/Type Permill
    baseWeight = 50000;
  });
  before("Minting assets", async function () {
    await mintAssetsToWallet(api, walletId1, sudoKey, [1, eth, usdc, usdt, dai]);
    await mintAssetsToWallet(api, walletId2, sudoKey, [1, eth, usdc, usdt, dai]);
  });
  after("Closing the connection", async function () {
    await api.disconnect();
  });
  it("Halborn Fix Validation", async function () {
    this.timeout(5 * 60 * 1000);
    poolId = await createConsProdPool(api, sudoKey, walletId1, eth, usdc, fee, baseWeight);
    poolId2 = await createConsProdPool(api, sudoKey, walletId1, usdt, eth, fee, baseWeight);
    const assetPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
      base: usdt,
      quote: usdc
    });
    const route = api.createType("Vec<u128>", [api.createType("u128", poolId), api.createType("u128", poolId2)]);
    await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.dexRouter.updateRoute(assetPair, route))
    );
    const badPool1 = await createConsProdPool(api, sudoKey, walletId2, badAsset, usdc, fee, baseWeight);
    const badPool2 = await createConsProdPool(api, sudoKey, walletId2, usdt, badAsset, fee, baseWeight);
    const badRoute = api.createType("Vec<u128>", [api.createType("u128", badPool1), api.createType("u128", badPool2)]);
    await sendAndWaitForSuccess(
      api,
      walletId2,
      api.events.dexRouter.RouteUpdated.is,
      api.tx.dexRouter.updateRoute(assetPair, badRoute)
      //Verify that the update route is permissioned
    ).catch(error => expect(error.message).to.contain("BadOrigin"));
  });
});
