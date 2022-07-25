import { ApiPromise } from "@polkadot/api";
import { txOracleAddAssetAndInfoSuccessTest, verifyOracleCreation } from "./testHandlers/addAssetAndInfoTests";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { expect } from "chai";
import { txOracleSetSignerSuccessTest } from "./testHandlers/setSignerTests";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { txOracleAddStakeSuccessTest } from "./testHandlers/addStakeTests";
import { txOracleSubmitPriceSuccessTest } from "./testHandlers/submitPriceTests";
import { waitForBlocks } from "@composable/utils/polkadotjs";
import BN from "bn.js";
import testConfiguration from "./test_configuration.json";

/**
 * This test suite contains tests for the HAL-02 issue
 * raised by Halborn in the security audit.
 * Audit Date: 19.02.22 - 29.04.22
 *
 * Issue description, Quote:
 * [...]
 * If the proposed price is not in the valid range from the newly chosen price (defined per asset),
 * Oracle, who submitted that price, would lose a portion of its tokens.
 *
 * However, the tokens are not subtracted from the staked balance but the free balance.
 * If there is no free balance in the user's account, slash would not be completed.
 *
 * For example a malicious Oracle might stake all of its tokens. Then Oracle
 * might send an invalid price proposal, manipulating the market. In such
 * a scenario, an Oracle pallet would not be able to punish the malicious Oracle,
 * who then may unstake the tokens and receive the initially staked tokens without penalties.
 *
 */
describe("HAL02 [Oracle] Tests", function () {
  if (!testConfiguration.enabledTests.HAL02) return;
  let api: ApiPromise;
  let assetID: number;
  let walletHAL02_1: KeyringPair,
    walletHAL02_2: KeyringPair,
    walletHAL02_3: KeyringPair,
    walletHAL02_4: KeyringPair,
    controllerWallet: KeyringPair,
    sudoKey: KeyringPair;

  before("HAL02: Setting up tests", async function () {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    controllerWallet = devWalletAlice.derive("/HAL02/oracleController");
    walletHAL02_1 = devWalletAlice.derive("/HAL02/oracleSigner1");
    walletHAL02_2 = devWalletAlice.derive("/HAL02/oracleSigner2");
    walletHAL02_3 = devWalletAlice.derive("/HAL02/oracleSigner3");
    walletHAL02_4 = devWalletAlice.derive("/HAL02/oracleSigner4");
    assetID = 1000;
  });

  before("HAL02: Providing funds", async function () {
    this.timeout(5 * 60 * 1000);
    await mintAssetsToWallet(api, controllerWallet, sudoKey, [1, assetID]);
    await mintAssetsToWallet(api, walletHAL02_1, sudoKey, [1, assetID]);
    await mintAssetsToWallet(api, walletHAL02_2, sudoKey, [1, assetID]);
    await mintAssetsToWallet(api, walletHAL02_3, sudoKey, [1, assetID]);
    await mintAssetsToWallet(api, walletHAL02_4, sudoKey, [1, assetID]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  it("HAL02: Creating oracle", async function () {
    this.timeout(2 * 60 * 1000);
    const assetId = api.createType("u128", assetID);
    const threshold = api.createType("Percent", 80);
    const minAnswers = api.createType("u32", 3);
    const maxAnswers = api.createType("u32", 5);
    const blockInterval = api.createType("u32", 6);
    const rewardWeight = api.createType("u128", 150000000);
    const slash = api.createType("u128", 100000000);
    const data = await txOracleAddAssetAndInfoSuccessTest(
      api,
      sudoKey,
      assetId,
      threshold,
      minAnswers,
      maxAnswers,
      blockInterval,
      rewardWeight,
      slash,
      true
    );
    await verifyOracleCreation(api, data, { threshold, minAnswers, maxAnswers, blockInterval, rewardWeight, slash });
  });

  describe("HAL02: Setting signers", function () {
    it("HAL02: Setting signer 1", async function () {
      this.timeout(2 * 60 * 1000);
      const {
        data: [resultAccount0, resultAccount1]
      } = await txOracleSetSignerSuccessTest(api, controllerWallet, walletHAL02_1).catch(function (exc) {
        return { data: [exc] }; /* We can't call this.skip() from here. */
      });

      if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use") return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
      expect(resultAccount0).to.not.be.an("Error");
      expect(resultAccount1).to.not.be.an("Error");
      expect(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_1.publicKey).toString());
      expect(resultAccount1.toString()).to.be.equal(
        api.createType("AccountId32", controllerWallet.publicKey).toString()
      );
    });

    it("HAL02: Setting signer 2", async function () {
      this.timeout(2 * 60 * 1000);
      const {
        data: [resultAccount0, resultAccount1]
      } = await txOracleSetSignerSuccessTest(api, walletHAL02_1, walletHAL02_2).catch(function (exc) {
        return { data: [exc] }; /* We can't call this.skip() from here. */
      });

      if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use") return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
      expect(resultAccount0).to.not.be.an("Error");
      expect(resultAccount1).to.not.be.an("Error");
      expect(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_2.publicKey).toString());
      expect(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_1.publicKey).toString());
    });

    it("HAL02: Setting signer 3", async function () {
      this.timeout(2 * 60 * 1000);
      const {
        data: [resultAccount0, resultAccount1]
      } = await txOracleSetSignerSuccessTest(api, walletHAL02_2, walletHAL02_3).catch(function (exc) {
        return { data: [exc] }; /* We can't call this.skip() from here. */
      });

      if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use") return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
      expect(resultAccount0).to.not.be.an("Error");
      expect(resultAccount1).to.not.be.an("Error");
      expect(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_3.publicKey).toString());
      expect(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_2.publicKey).toString());
    });

    it("HAL02: Setting signer 4", async function () {
      this.timeout(2 * 60 * 1000);
      const {
        data: [resultAccount0, resultAccount1]
      } = await txOracleSetSignerSuccessTest(api, walletHAL02_3, walletHAL02_4).catch(function (exc) {
        return { data: [exc] }; /* We can't call this.skip() from here. */
      });

      if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use") return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
      expect(resultAccount0).to.not.be.an("Error");
      expect(resultAccount1).to.not.be.an("Error");
      expect(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_4.publicKey).toString());
      expect(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_3.publicKey).toString());

      // We need to further elect a new signer,
      // else signer 4 won't be able to add its stake.
      const {
        data: [result2Account0, result2Account1]
      } = await txOracleSetSignerSuccessTest(api, walletHAL02_4, controllerWallet).catch(function (exc) {
        return { data: [exc] }; /* We can't call this.skip() from here. */
      });
      expect(result2Account0).to.not.be.an("Error");
      expect(result2Account1).to.not.be.an("Error");
      expect(result2Account0.toString()).to.be.equal(
        api.createType("AccountId32", controllerWallet.publicKey).toString()
      );
      expect(result2Account1.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_4.publicKey).toString());
    });
  });

  describe("HAL02: Adding stakes", function () {
    it("HAL02: Adding stakes", async function () {
      this.timeout(2 * 60 * 1000);
      const stake = api.createType("u128", 2500000000000);
      const [
        {
          data: [result]
        }
      ] = await Promise.all([
        txOracleAddStakeSuccessTest(api, walletHAL02_1, stake),
        txOracleAddStakeSuccessTest(api, walletHAL02_2, stake),
        txOracleAddStakeSuccessTest(api, walletHAL02_3, stake),
        txOracleAddStakeSuccessTest(api, walletHAL02_4, stake)
      ]);
      expect(result).to.not.be.an("Error");
      expect(result.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_2.publicKey).toString());
    });
  });

  describe("HAL02: Test Scenarios", function () {
    this.retries(0);
    it("HAL02: Scenario 1: Oracle stake of malicious actor should get slashed", async function () {
      this.timeout(10 * 60 * 1000);

      const correctPrice = api.createType("u128", 100);
      const maliciousPrice = api.createType("u128", 900);
      const asset = api.createType("u128", assetID);

      const balanceWallet1BeforeTransaction = new BN(
        (await api.rpc.assets.balanceOf(asset.toString(), walletHAL02_1.publicKey)).toString()
      );
      const balanceWallet2BeforeTransaction = new BN(
        (await api.rpc.assets.balanceOf(asset.toString(), walletHAL02_2.publicKey)).toString()
      );
      const balanceWallet3BeforeTransaction = new BN(
        (await api.rpc.assets.balanceOf(asset.toString(), walletHAL02_3.publicKey)).toString()
      );
      const balanceWallet4BeforeTransaction = new BN(
        (await api.rpc.assets.balanceOf(asset.toString(), walletHAL02_4.publicKey)).toString()
      );

      const oracleStakeWallet1BeforeTransaction = new BN(
        (await api.query.oracle.oracleStake(walletHAL02_1.publicKey)).toString()
      );
      const oracleStakeWallet2BeforeTransaction = new BN(
        (await api.query.oracle.oracleStake(walletHAL02_2.publicKey)).toString()
      );
      const oracleStakeWallet3BeforeTransaction = new BN(
        (await api.query.oracle.oracleStake(walletHAL02_3.publicKey)).toString()
      );
      const oracleStakeWallet4BeforeTransaction = new BN(
        (await api.query.oracle.oracleStake(walletHAL02_4.publicKey)).toString()
      );

      // Submit 2 correct & 2 malicious prices.
      await Promise.all([
        txOracleSubmitPriceSuccessTest(api, walletHAL02_1, correctPrice, asset),
        txOracleSubmitPriceSuccessTest(api, walletHAL02_2, correctPrice, asset),
        txOracleSubmitPriceSuccessTest(api, walletHAL02_3, maliciousPrice, asset)
      ]).then(async function ([
        {
          data: [result1AccountID, result1AssetID, result1ReportedPrice]
        },
        {
          data: [result2AccountID, result2AssetID, result2ReportedPrice]
        },
        {
          data: [result3AccountID, result3AssetID, result3ReportedPrice]
        }
      ]) {
        expect(result1AssetID.toNumber())
          .to.be.equal(result2AssetID.toNumber())
          .to.be.equal(result3AssetID.toNumber())
          .to.be.equal(asset.toNumber());

        expect(result1ReportedPrice.toNumber())
          .to.be.equal(result2ReportedPrice.toNumber())
          .to.be.equal(correctPrice.toNumber());
        expect(result3ReportedPrice.toNumber()).to.be.equal(maliciousPrice.toNumber());

        expect(result1AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL02_1.publicKey).toString());
        expect(result2AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL02_2.publicKey).toString());
        expect(result3AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL02_3.publicKey).toString());

        // Waiting a few blocks to make sure the slashing took place.
        await waitForBlocks(api, 3);

        const balanceWallet1AfterTransaction = new BN(
          (await api.rpc.assets.balanceOf(asset.toString(), walletHAL02_1.publicKey)).toString()
        );
        const balanceWallet2AfterTransaction = new BN(
          (await api.rpc.assets.balanceOf(asset.toString(), walletHAL02_2.publicKey)).toString()
        );
        const balanceWallet3AfterTransaction = new BN(
          (await api.rpc.assets.balanceOf(asset.toString(), walletHAL02_3.publicKey)).toString()
        );
        const balanceWallet4AfterTransaction = new BN(
          (await api.rpc.assets.balanceOf(asset.toString(), walletHAL02_4.publicKey)).toString()
        );

        const oracleStakeWallet1AfterTransaction = new BN(
          (await api.query.oracle.oracleStake(walletHAL02_1.publicKey)).toString()
        );
        const oracleStakeWallet2AfterTransaction = new BN(
          (await api.query.oracle.oracleStake(walletHAL02_2.publicKey)).toString()
        );
        const oracleStakeWallet3AfterTransaction = new BN(
          (await api.query.oracle.oracleStake(walletHAL02_3.publicKey)).toString()
        );
        const oracleStakeWallet4AfterTransaction = new BN(
          (await api.query.oracle.oracleStake(walletHAL02_4.publicKey)).toString()
        );

        // Malicious price providers oracle stash should get slashed.
        expect(oracleStakeWallet3AfterTransaction).to.be.bignumber.lessThan(oracleStakeWallet3BeforeTransaction);
        // The other ones shouldn't.
        expect(oracleStakeWallet1AfterTransaction).to.be.bignumber.equal(oracleStakeWallet1BeforeTransaction);
        expect(oracleStakeWallet2AfterTransaction).to.be.bignumber.equal(oracleStakeWallet2BeforeTransaction);
        expect(oracleStakeWallet4AfterTransaction).to.be.bignumber.equal(oracleStakeWallet4BeforeTransaction);

        // Wallet Balances shouldn't get slashed.
        expect(balanceWallet1BeforeTransaction).to.be.bignumber.equal(balanceWallet1AfterTransaction);
        expect(balanceWallet2BeforeTransaction).to.be.bignumber.equal(balanceWallet2AfterTransaction);
        expect(balanceWallet3BeforeTransaction).to.be.bignumber.equal(balanceWallet3AfterTransaction);
        expect(balanceWallet4BeforeTransaction).to.be.bignumber.equal(balanceWallet4AfterTransaction);
      });
    });
  });
});
