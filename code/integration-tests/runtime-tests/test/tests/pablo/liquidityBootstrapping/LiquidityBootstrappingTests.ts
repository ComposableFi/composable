import { KeyringPair } from "@polkadot/keyring/types";
import { mintAssetsToWallet, Pica } from "@composable/utils/mintingHelper";
import { expect } from "chai";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { ApiPromise } from "@polkadot/api";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import {
  addFundsToThePool,
  buyFromPool,
  createLBPool,
  createMultipleLBPools,
  rpcPriceFor,
  sellToPool,
  swapTokenPairs
} from "@composabletests/tests/pablo/testHandlers/pabloTestHelper";
import testConfiguration from "@composabletests/tests/pablo/liquidityBootstrapping/test_configuration.json";
import pabloTestConfiguration from "../testHandlers/test_configuration.json";

/**
 * This suite includes tests for the Liquidity Bootstrapping Pool.
 * Tested functionalities are:
 * CreatePool - AddLiquidity - Buy - Sell - Swap - RemoveLiquidity with basic calculations
 */

describe.only("LiquidityBootstrapping Pool Test Suite", function () {
  if (!pabloTestConfiguration.liquidityBootstrappingTests.enabled) {
    console.log("Liquidity Bootstrapping Tests are being skipped...");
    return;
  }
  this.timeout(3 * 60 * 1000);
  let api: ApiPromise;
  let poolId1: number, poolId2: number, feeRate: number, ownerFeeRate: number, protocolFeeRate: number;
  let walletId1: KeyringPair, walletId2: KeyringPair, sudoKey: KeyringPair;
  let baseAssetId: number, quoteAssetId: number, quoteAssetId2: number;
  let baseAmount: bigint, quoteAmount: bigint;
  let startTime: number, endTime: number, initialWeight: number, finalWeight: number;

  before("Given that users have sufficient balance", async function () {
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice, devWalletEve, devWalletFerdie } = getDevWallets(newKeyring);
    walletId1 = devWalletFerdie.derive("/test/lbpTests/wallet1");
    walletId2 = devWalletEve.derive("/test/lbpTests/wallet2");
    sudoKey = devWalletAlice;
    baseAssetId = 15;
    quoteAssetId = 20;
    quoteAssetId2 = 25;
    //Gets latest block number
    const latestHead = await api.rpc.chain.getFinalizedHead();
    const latestBlock = (await api.rpc.chain.getHeader(latestHead)).number;
    startTime = latestBlock.toNumber() + 24;
    //Should be a number between 216000 - 7200
    endTime = startTime + 7300;
    //Max Initial weight is 95%, here it is passed as 50%
    initialWeight = 900000;
    //Min final weight is 5%, here it is as passed as 6%
    finalWeight = 60000;
    //Sets 1% fee
    feeRate = 10000;
    ownerFeeRate = 200000;
    protocolFeeRate = 600000;
    baseAmount = Pica(3000);
    quoteAmount = Pica(10000);
    await mintAssetsToWallet(api, walletId1, sudoKey, [1, baseAssetId, quoteAssetId]);
    await mintAssetsToWallet(api, walletId2, sudoKey, [1, baseAssetId, quoteAssetId, quoteAssetId2]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  describe("LiquidityBootstrapping CreatePool and AddLiquidity Tests", async function () {
    if (!testConfiguration.enabledTests.createPoolAndAddLiquidityTests.enabled) {
      console.log("LiquidityBootstrapping createPool and addLiquidity tests are being skipped...");
      return;
    }
    this.timeout(2 * 60 * 1000);

    it("User1 can create a LBP", async function () {
      const result = await createLBPool(
        api,
        sudoKey,
        walletId1,
        baseAssetId,
        quoteAssetId,
        startTime,
        endTime,
        initialWeight,
        finalWeight,
        feeRate,
        ownerFeeRate,
        protocolFeeRate
      );
      poolId1 = result.resultPoolId;
      expect(poolId1).to.be.a("number");
    });

    it("Users can create multiple LB Pools with the same token pairs", async function () {
      const result = await createLBPool(
        api,
        sudoKey,
        walletId1,
        baseAssetId,
        quoteAssetId,
        startTime,
        endTime,
        initialWeight,
        finalWeight,
        feeRate,
        ownerFeeRate,
        protocolFeeRate
      );
      expect(result.resultPoolId).to.be.a("number");
    });

    it("Users can add liquidity to the LB pool before the sale started", async function () {
      const result = await addFundsToThePool(api, poolId1, walletId1, baseAmount, quoteAmount);
      expect(result.baseAdded.toBigInt()).to.be.equal(baseAmount);
      expect(result.quoteAdded.toBigInt()).to.be.equal(quoteAmount);
    });

    it("Given that the pool created, Owner can enable twap", async function () {
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.pablo.enableTwap(api.createType("u128", poolId1)))
      );
      expect(result.isOk).to.be.true;
    });

    it("Given that the LB pool has liquidity in the pool, before the sale started, it returns 0 spot price", async function () {
      const rpcRes = await rpcPriceFor(
        api,
        api.createType("PalletPabloPoolId", poolId1),
        api.createType("CustomRpcCurrencyId", baseAssetId),
        api.createType("CustomRpcCurrencyId", quoteAssetId)
      );
      expect(rpcRes.spotPrice.toString()).to.be.equal("0");
    });

    it("Given that the pool has sufficient liquidity, user1 can't buy from the pool before the sale starts", async function () {
      await buyFromPool(api, poolId1, walletId1, baseAssetId, Pica(5)).catch(e =>
        expect(e.message).to.contain("InvalidSaleState")
      );
    });

    it("Users can create multiple LB Pools with random valid parameters", async function () {
      const prePoolCount = (await api.query.pablo.poolCount()).toNumber();
      await createMultipleLBPools(api, walletId1);
      expect((await api.query.pablo.poolCount()).toNumber()).to.be.equal(500 + prePoolCount);
    });

    it(
      "The users can't create LB Pools with invalid params " +
        "{initial weight>95, end weight<5, saleDuration<7200, saleDuration>216000",
      async function () {
        this.timeout(3 * 60 * 1000);
        // ToDo: Update accordingly!
        // There have been changes to the allowed parameters of pools.
        // Please verify & update accordingly.
        this.skip();
        const weights = [950001, 49999];
        const durations = [7100, 216001];
        for (const weight of weights) {
          // @ts-ignore // ToDo: Remove! See above.
          await createLBPool(
            api,
            sudoKey,
            walletId1,
            baseAssetId,
            quoteAssetId,
            startTime,
            endTime,
            weight,
            50001,
            feeRate,
            ownerFeeRate,
            protocolFeeRate
          ).catch(e => expect(e.message).to.contain("Other"));
        }
        // @ts-ignore // ToDo: Remove! See above.
        for (const duration of durations) {
          // @ts-ignore // ToDo: Remove! See above.
          await createLBPool(
            api,
            sudoKey,
            walletId1,
            baseAssetId,
            quoteAssetId,
            startTime,
            startTime + duration,
            initialWeight,
            finalWeight,
            feeRate,
            ownerFeeRate,
            protocolFeeRate
          ).catch(e => expect(e.message).to.contain("Other"));
        }
      }
    );
  });

  describe("LiquidityBootstrapping buy sell and swap tests", async function () {
    if (!testConfiguration.enabledTests.buySellAndSwapTests.enabled) {
      console.log("LiquidityBootstrapping buy,sell and swap tests are being skipped...");
      return;
    }
    this.timeout(3 * 60 * 1000);

    it("User2 can buy from the pool once the sale started", async function () {
      const result = await buyFromPool(api, poolId1, walletId2, quoteAssetId, Pica(30));
      expect(result.baseAmount.toBn()).to.be.bignumber.closeTo(Pica(30).toString(), Pica(10).toString());
    });

    it("Given that users have sufficient funds, user1 can't buy an asset not listed in the pool", async function () {
      await buyFromPool(api, poolId1, walletId2, quoteAssetId2, Pica(10)).catch(error =>
        expect(error.message).to.contain("InvalidAsset")
      );
    });

    it("User1 can sell to the pool once the sale started", async function () {
      const result = await sellToPool(api, poolId1, walletId1, baseAssetId, Pica(50));
      expect(result.toString()).to.be.equal(api.createType("AccountId32", walletId1.address).toString());
    });

    it("User can't buy more than the amount available in the pool", async function () {
      await buyFromPool(api, poolId2, walletId2, baseAssetId, Pica("4000")).catch(error =>
        expect(error.message).to.contain("InvalidAsset")
      );
    });

    it("User1 can swap from the pool once the sale started", async function () {
      const result = await swapTokenPairs(api, poolId1, walletId1, baseAssetId, quoteAssetId, Pica(60));
      expect(result.returnedQuoteAmount.toBigInt()).to.be.equal(Pica(60));
    });

    it("User2 can't swap from the pool with nonexisting assetId's", async function () {
      await swapTokenPairs(api, poolId1, walletId1, baseAssetId, quoteAssetId2, Pica(50)).catch(error =>
        expect(error.message).to.contain("PairMismatch")
      );
    });
  });
});
