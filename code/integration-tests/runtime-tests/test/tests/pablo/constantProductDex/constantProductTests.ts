import pabloTestConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { mintAssetsToWallet, Pica } from "@composable/utils/mintingHelper";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { ApiPromise } from "@polkadot/api";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { expect } from "chai";


const DEFAULT_FEE = 10_000; // 1 Percent

describe("tx.constantProductDex Tests", function() {
  if (!pabloTestConfiguration.enabledTests.enabled) {
    console.log("Constant Product Tests are being skipped...");
    return;
  }
  this.timeout(3 * 60 * 1000);
  let api: ApiPromise;
  let sudoKey: KeyringPair, poolOwnerWallet:KeyringPair, walletLpProvider1: KeyringPair, walletTrader1: KeyringPair;
  let fee: number;
  let baseWeight: number;
  let baseAmount: bigint, quoteAmount: bigint;
  let poolId1:number;

  before("Initialize variables", async function() {
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice, devWalletEve, devWalletFerdie } = getDevWallets(newKeyring);

    sudoKey = devWalletAlice;
    poolOwnerWallet = devWalletFerdie.derive("/test/pablo/pool/owner");
    walletLpProvider1 = devWalletFerdie.derive("/test/pablo/lp/provider/1");
    walletTrader1 = devWalletFerdie.derive("/test/pablo/trader/1");

    baseAmount = Pica(250000);
    quoteAmount = Pica(250000);
    //sets the weight of the asset pairs to 50.00%/Type Permill
    baseWeight = 500000;
  });

  before("Minting assets", async function() {
    // await mintAssetsToWallet(api, poolOwnerWallet, sudoKey, [1]);
    // await mintAssetsToWallet(api, walletLpProvider1, sudoKey, [1]);
    // await mintAssetsToWallet(api, walletTrader1, sudoKey, [1]);
  });

  after("Closing the connection", async function() {
    await api.disconnect();
  });

  describe("1. Pool creation", function() {
    it("#1.1  I can, as sudo, create a new Pablo Constant Product pool.", async function() {
      this.timeout(2*60*1000);

      const owner = api.createType("AccountId32", poolOwnerWallet.publicKey);
      const poolConfiguration = api.createType("PalletPabloPoolInitConfiguration", {
        DualAssetConstantProduct: {
          //pool: {
            owner: owner,
            assetsWeights: api.createType("BTreeMap<u128, Permill>", {
              "1": 500_000,
              "131": 500_000
            }),
            fee: api.createType("Permill", 10_000)
          }
      //}
        });

      const {data: [resultPoolId, resultOwner, resultAssets]} = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.pablo.PoolCreated.is,
        api.tx.sudo.sudo(api.tx.pablo.create(poolConfiguration))
      );
      poolId1 = resultPoolId.toNumber();
      expect(resultOwner.toString()).to.be.equal(owner.toString());
      console.debug(resultAssets.toHuman());
    });

    it("#1.2  I can, as sudo, create a new Pablo Constant Product pool, for assets which already belong to an existing pool.");

    it("#1.3  User wallets can not create new Pablo Constant Product pools.");

    it("#1.4  I can, as sudo, create a new Pablo Constant Product pool with 0 fees.");

    it("#1.5  I can not, as sudo, create a new Pablo Constant Product pool with fees greater than 100%.");

    it("#1.6  I can not, as sudo, create a new Pablo Constant Product pool with the base asset being equal to the quote asset.");
  });

  describe("2. Providing liquidity", function() {
    it("#2.1  I can provide liquidity to the newly created pool. #1.1");

    it("#2.2  I can transfer my LP tokens to another user.");

    it("#2.3  I can not add only the base or quote asset as liquidity");

    it("#2.4  I can not add liquidity amounts of 0.");

    it("#2.5  I can not add liquidity without respecting the pools ratio.");

    it("#2.6  I can add liquidity with a defined `minMintAmount`.");

    it("#2.7  I can add liquidity to a pool with already available liquidity.");
  });

  describe("3. Removing liquidity", function() {
    it("#3.1  I can not remove more liquidity than the amount equivalent to my LP token amount.");

    it("#3.2  I can not remove liquidity amounts of 0.");

    it("#3.3  I can remove liquidity based on LP tokens which were sent to me.");

    it("#3.4  I can not remove liquidity from a pool by using the LP tokens of the different pool.");

    it("#3.5  I can remove earlier provided liquidity.");

    it("#3.6  I can remove earlier provided liquidity with defined `minReceive`.");
  });

  describe("4. Trading", function() {
    it("#4.1  I can not buy an amount more than available liquidity.");

    it("#4.2  I can not buy  an asset which isn't part of the pool.");

    it("#4.3  I can not swap in a pool with assets that aren't listed in that pool.");

    it("#4.4  I can buy an amount, and provided by the amounts i want to give in it'll adjusted by the `outGivenIn` formula.");

    it("#4.5  I can buy an amount, and provided by the amount i want to get out it'll be adjusted by the `inGivenOut` formula.");

    it("#4.6  I can not buy 0 amounts of any asset.");

    it("#4.7  I can not buy all of the available liquidity of a pool.");

    it("#4.8  I can not buy with the base asset being the same as the quote asset.");

    it("#4.9  I can not buy in a pool without providing liquidity.");

    it("#4.10 I can not swap in a pool without liquidity.");

    it("#4.11 I can not buy or swap in a pool that doesn't exist.");

    it("#4.12 I can not buy or swap with the minimum amount requested  greater than the trade would give.");

    it("#4.13 I can buy a huge amount with very high slippage.");

    it("#4.14 I can swap a huge amount with very high slippage.");

    it("#4.15 I can swap an amount, provided by the amount i want to give in, and it'll be adjusted by the `outGivenIn` formula.");

    it("#4.16 I can swap an amount, provided by the amount i want to get out, and it'll be adjusted by the `inGivenOut` formula.");

    it("#4.17 I can buy in the pool with 0 fees & pay 0 fees.");

    it("#4.18 I can swap in the pool with 0 fees & pay 0 fees.");
  });

});
