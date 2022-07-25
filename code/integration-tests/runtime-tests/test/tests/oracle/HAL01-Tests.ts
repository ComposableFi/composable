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

const getOracleStake = async (api: ApiPromise, wallet: KeyringPair): Promise<BN> =>
  new BN((await api.query.oracle.oracleStake(wallet.publicKey)).toString());

/**
 * This test suite contains tests for the HAL-01 issue
 * raised by Halborn in the security audit.
 * Audit Date: 19.02.22 - 29.04.22
 *
 * Issue description, Quote:
 * [...]
 * To prevent malicious Oracles from manipulating the asset's price,
 * every proposal which would not be in the acceptable range results
 * in a slash of Oracle balance. However, two scenarios are possible
 * where this mechanism can be exploited.
 *
 * Suppose exactly half of the proposed prices would be malicious,
 * i.e., substantially increasing of decreasing an asset's price.
 * In that case, all Oracles might get slashes, regardless if they
 * submitted a plausible price or not.
 *
 * On the other hand, if most of the proposed prices were malicious,
 * then such a situation would result in legitimate Oracles getting slashed.
 *
 */
describe("HAL01 [Oracle] Tests", function () {
  if (!testConfiguration.enabledTests.HAL01) return;
  let api: ApiPromise;
  let assetID: number;
  let walletHAL01_1: KeyringPair,
    walletHAL01_2: KeyringPair,
    walletHAL01_3: KeyringPair,
    walletHAL01_4: KeyringPair,
    controllerWallet: KeyringPair,
    sudoKey: KeyringPair;

  before("HAL01: Setting up tests", async function () {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    controllerWallet = devWalletAlice.derive("/HAL01/oracleController");
    walletHAL01_1 = devWalletAlice.derive("/HAL01/oracleSigner1");
    walletHAL01_2 = devWalletAlice.derive("/HAL01/oracleSigner2");
    walletHAL01_3 = devWalletAlice.derive("/HAL01/oracleSigner3");
    walletHAL01_4 = devWalletAlice.derive("/HAL01/oracleSigner4");
    assetID = 1001;
  });

  before("HAL01: Providing funds", async function () {
    this.timeout(5 * 60 * 1000);
    await mintAssetsToWallet(api, controllerWallet, sudoKey, [1, assetID]);
    await mintAssetsToWallet(api, walletHAL01_1, sudoKey, [1, assetID]);
    await mintAssetsToWallet(api, walletHAL01_2, sudoKey, [1, assetID]);
    await mintAssetsToWallet(api, walletHAL01_3, sudoKey, [1, assetID]);
    await mintAssetsToWallet(api, walletHAL01_4, sudoKey, [1, assetID]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  it("HAL01: Creating oracle", async function () {
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

  describe("HAL01: Setting signers", function () {
    it("HAL01: Setting signer 1", async function () {
      this.timeout(2 * 60 * 1000);
      const {
        data: [resultAccount0, resultAccount1]
      } = await txOracleSetSignerSuccessTest(api, controllerWallet, walletHAL01_1).catch(function (exc) {
        return { data: [exc] }; /* We can't call this.skip() from here. */
      });

      if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use") return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
      expect(resultAccount0).to.not.be.an("Error");
      expect(resultAccount1).to.not.be.an("Error");
      expect(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_1.publicKey).toString());
      expect(resultAccount1.toString()).to.be.equal(
        api.createType("AccountId32", controllerWallet.publicKey).toString()
      );
    });

    it("HAL01: Setting signer 2", async function () {
      this.timeout(2 * 60 * 1000);
      const {
        data: [resultAccount0, resultAccount1]
      } = await txOracleSetSignerSuccessTest(api, walletHAL01_1, walletHAL01_2).catch(function (exc) {
        return { data: [exc] }; /* We can't call this.skip() from here. */
      });

      if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use") return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
      expect(resultAccount0).to.not.be.an("Error");
      expect(resultAccount1).to.not.be.an("Error");
      expect(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_2.publicKey).toString());
      expect(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_1.publicKey).toString());
    });

    it("HAL01: Setting signer 3", async function () {
      this.timeout(2 * 60 * 1000);
      const {
        data: [resultAccount0, resultAccount1]
      } = await txOracleSetSignerSuccessTest(api, walletHAL01_2, walletHAL01_3).catch(function (exc) {
        return { data: [exc] }; /* We can't call this.skip() from here. */
      });

      if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use") return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
      expect(resultAccount0).to.not.be.an("Error");
      expect(resultAccount1).to.not.be.an("Error");
      expect(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_3.publicKey).toString());
      expect(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_2.publicKey).toString());
    });

    it("HAL01: Setting signer 4", async function () {
      this.timeout(2 * 60 * 1000);
      const {
        data: [resultAccount0, resultAccount1]
      } = await txOracleSetSignerSuccessTest(api, walletHAL01_3, walletHAL01_4).catch(function (exc) {
        return { data: [exc] }; /* We can't call this.skip() from here. */
      });

      if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use") return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
      expect(resultAccount0).to.not.be.an("Error");
      expect(resultAccount1).to.not.be.an("Error");
      expect(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_4.publicKey).toString());
      expect(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_3.publicKey).toString());

      // We need to further elect a new signer,
      // else signer 4 won't be able to add its stake.
      const {
        data: [result2Account0, result2Account1]
      } = await txOracleSetSignerSuccessTest(api, walletHAL01_4, controllerWallet).catch(function (exc) {
        return { data: [exc] }; /* We can't call this.skip() from here. */
      });
      expect(result2Account0).to.not.be.an("Error");
      expect(result2Account1).to.not.be.an("Error");
      expect(result2Account0.toString()).to.be.equal(
        api.createType("AccountId32", controllerWallet.publicKey).toString()
      );
      expect(result2Account1.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_4.publicKey).toString());
    });
  });

  describe("HAL01: Adding stakes", function () {
    it("HAL01: Adding stakes", async function () {
      this.timeout(2 * 60 * 1000);
      const stake = api.createType("u128", 25000000000000);
      const [
        {
          data: [result]
        },
        {
          data: [result2]
        },
        {
          data: [result3]
        },
        {
          data: [result4]
        }
      ] = await Promise.all([
        txOracleAddStakeSuccessTest(api, walletHAL01_1, stake),
        txOracleAddStakeSuccessTest(api, walletHAL01_2, stake),
        txOracleAddStakeSuccessTest(api, walletHAL01_3, stake),
        txOracleAddStakeSuccessTest(api, walletHAL01_4, stake)
      ]);
      expect(result).to.not.be.an("Error");
      expect(result.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_2.publicKey).toString());
      expect(result2).to.not.be.an("Error");
      expect(result2.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_3.publicKey).toString());
      expect(result3).to.not.be.an("Error");
      expect(result3.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_4.publicKey).toString());
      expect(result4).to.not.be.an("Error");
      expect(result4.toString()).to.be.equal(api.createType("AccountId32", controllerWallet.publicKey).toString());
    });
  });

  describe("HAL01: Test Scenarios", function () {
    this.retries(0);
    it("HAL01: Scenario 1: 50% of Oracles are malicious", async function () {
      this.timeout(10 * 60 * 1000);

      const correctPrice = api.createType("u128", 100);
      const maliciousPrice = api.createType("u128", 900);
      const asset = api.createType("u128", assetID);

      const [
        oracleStakeWallet1BeforeTransaction,
        oracleStakeWallet2BeforeTransaction,
        oracleStakeWallet3BeforeTransaction,
        oracleStakeWallet4BeforeTransaction
      ] = await Promise.all([
        getOracleStake(api, walletHAL01_1),
        getOracleStake(api, walletHAL01_2),
        getOracleStake(api, walletHAL01_3),
        getOracleStake(api, walletHAL01_4)
      ]);
      expect(oracleStakeWallet1BeforeTransaction).to.be.bignumber.greaterThan("0");
      expect(oracleStakeWallet2BeforeTransaction).to.be.bignumber.greaterThan("0");
      expect(oracleStakeWallet3BeforeTransaction).to.be.bignumber.greaterThan("0");
      expect(oracleStakeWallet4BeforeTransaction).to.be.bignumber.greaterThan("0");

      // Submit 2 correct & 2 malicious prices.
      await Promise.all([
        txOracleSubmitPriceSuccessTest(api, walletHAL01_1, correctPrice, asset),
        txOracleSubmitPriceSuccessTest(api, walletHAL01_2, correctPrice, asset),
        txOracleSubmitPriceSuccessTest(api, walletHAL01_3, maliciousPrice, asset),
        txOracleSubmitPriceSuccessTest(api, walletHAL01_4, maliciousPrice, asset)
      ]).then(async function ([
        {
          data: [result1AccountID, result1AssetID, result1ReportedPrice]
        },
        {
          data: [result2AccountID, result2AssetID, result2ReportedPrice]
        },
        {
          data: [result3AccountID, result3AssetID, result3ReportedPrice]
        },
        {
          data: [result4AccountID, result4AssetID, result4ReportedPrice]
        }
      ]) {
        expect(result1AssetID.toNumber())
          .to.be.equal(result2AssetID.toNumber())
          .to.be.equal(result3AssetID.toNumber())
          .to.be.equal(result4AssetID.toNumber())
          .to.be.equal(asset.toNumber());

        expect(result1ReportedPrice.toNumber())
          .to.be.equal(result2ReportedPrice.toNumber())
          .to.be.equal(correctPrice.toNumber());
        expect(result3ReportedPrice.toNumber())
          .to.be.equal(result4ReportedPrice.toNumber())
          .to.be.equal(maliciousPrice.toNumber());

        expect(result1AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL01_1.publicKey).toString());
        expect(result2AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL01_2.publicKey).toString());
        expect(result3AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL01_3.publicKey).toString());
        expect(result4AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL01_4.publicKey).toString());

        const [
          oracleStakeWallet1AfterTransaction,
          oracleStakeWallet2AfterTransaction,
          oracleStakeWallet3AfterTransaction,
          oracleStakeWallet4AfterTransaction
        ] = await Promise.all([
          getOracleStake(api, walletHAL01_1),
          getOracleStake(api, walletHAL01_2),
          getOracleStake(api, walletHAL01_3),
          getOracleStake(api, walletHAL01_4)
        ]);

        // Nobody should get slashed.
        expect(oracleStakeWallet1BeforeTransaction).to.be.bignumber.equal(oracleStakeWallet1AfterTransaction);
        expect(oracleStakeWallet2BeforeTransaction).to.be.bignumber.equal(oracleStakeWallet2AfterTransaction);
        expect(oracleStakeWallet3BeforeTransaction).to.be.bignumber.equal(oracleStakeWallet3AfterTransaction);
        expect(oracleStakeWallet4BeforeTransaction).to.be.bignumber.equal(oracleStakeWallet4AfterTransaction);
      });
    });

    it("HAL01: Scenario 2: >50% of Oracles are malicious", async function () {
      this.timeout(10 * 60 * 1000);

      const correctPrice = api.createType("u128", 100);
      const maliciousPrice = api.createType("u128", 900);
      const asset = api.createType("u128", assetID);

      const [
        oracleStakeWallet1BeforeTransaction,
        oracleStakeWallet2BeforeTransaction,
        oracleStakeWallet3BeforeTransaction,
        oracleStakeWallet4BeforeTransaction
      ] = await Promise.all([
        getOracleStake(api, walletHAL01_1),
        getOracleStake(api, walletHAL01_2),
        getOracleStake(api, walletHAL01_3),
        getOracleStake(api, walletHAL01_4)
      ]);
      expect(oracleStakeWallet1BeforeTransaction).to.be.bignumber.greaterThan("0");
      expect(oracleStakeWallet2BeforeTransaction).to.be.bignumber.greaterThan("0");
      expect(oracleStakeWallet3BeforeTransaction).to.be.bignumber.greaterThan("0");
      expect(oracleStakeWallet4BeforeTransaction).to.be.bignumber.greaterThan("0");

      await waitForBlocks(api, 6);

      // Submit 1 correct & 2 malicious prices.
      await Promise.all([
        txOracleSubmitPriceSuccessTest(api, walletHAL01_1, correctPrice, asset),
        txOracleSubmitPriceSuccessTest(api, walletHAL01_3, maliciousPrice, asset),
        txOracleSubmitPriceSuccessTest(api, walletHAL01_4, maliciousPrice, asset)
      ]).then(async function ([
        {
          data: [result1AccountID, result1AssetID, result1ReportedPrice]
        },
        {
          data: [result3AccountID, result3AssetID, result3ReportedPrice]
        },
        {
          data: [result4AccountID, result4AssetID, result4ReportedPrice]
        }
      ]) {
        expect(result1AssetID.toNumber())
          .to.be.equal(result3AssetID.toNumber())
          .to.be.equal(result4AssetID.toNumber())
          .to.be.equal(asset.toNumber());

        expect(result1ReportedPrice.toNumber()).to.be.equal(correctPrice.toNumber());
        expect(result3ReportedPrice.toNumber())
          .to.be.equal(result4ReportedPrice.toNumber())
          .to.be.equal(maliciousPrice.toNumber());

        expect(result1AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL01_1.publicKey).toString());
        expect(result3AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL01_3.publicKey).toString());
        expect(result4AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL01_4.publicKey).toString());

        const [
          oracleStakeWallet1AfterTransaction,
          oracleStakeWallet2AfterTransaction,
          oracleStakeWallet3AfterTransaction,
          oracleStakeWallet4AfterTransaction
        ] = await Promise.all([
          getOracleStake(api, walletHAL01_1),
          getOracleStake(api, walletHAL01_2),
          getOracleStake(api, walletHAL01_3),
          getOracleStake(api, walletHAL01_4)
        ]);

        // Nobody should get slashed.
        expect(oracleStakeWallet1BeforeTransaction).to.be.bignumber.equal(oracleStakeWallet1AfterTransaction);
        expect(oracleStakeWallet2BeforeTransaction).to.be.bignumber.equal(oracleStakeWallet2AfterTransaction);
        expect(oracleStakeWallet3BeforeTransaction).to.be.bignumber.equal(oracleStakeWallet3AfterTransaction);
        expect(oracleStakeWallet4BeforeTransaction).to.be.bignumber.equal(oracleStakeWallet4AfterTransaction);
      });
    });
  });
});
