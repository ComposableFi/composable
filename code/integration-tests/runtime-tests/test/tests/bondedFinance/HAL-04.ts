import { getNewConnection } from "@composable/utils/connectionHelper";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { expect } from "chai";
import testConfiguration from "@composabletests/tests/bondedFinance/test_configuration.json";
import { txBondedFinanceOfferSuccessTest } from "@composabletests/tests/bondedFinance/testHandlers/offerTests";

/**
 * Contains the test for the HAL-04 issue.
 *
 * HAL-04 is about creation a bonded finance offer,
 * with the same asset for staking & reward.
 */
describe("1. Bonded Finance Audit - HAL-04 Tests", function () {
  if (!testConfiguration.enabledTests.HAL04) return;
  let api: ApiPromise;
  let walletAlice: KeyringPair;

  before("Setting up tests", async function () {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice } = getDevWallets(newKeyring);
    walletAlice = devWalletAlice;
  });

  before("mint assets into the wallet", async function () {
    this.timeout(2 * 60 * 1000);
    await mintAssetsToWallet(api, walletAlice, walletAlice, [4]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  /**
   * bondedFinance.offer(...) Success Tests
   */
  describe("HAL-04 Test Cases", function () {
    if (!testConfiguration.enabledTests.offer_bond__success.enabled) return;
    // Timeout set to 2 minutes
    this.timeout(2 * 60 * 1000);

    it("Should not be able to create offer with same asset", async function () {
      const requestParameters = {
        beneficiary: walletAlice.publicKey,
        asset: api.createType("u128", 4),
        bondPrice: api.createType("u128", 100000000000000),
        nbOfBonds: api.createType("u128", 10),
        maturity: { Finite: { returnIn: api.createType("u32", 16) } },
        reward: {
          asset: api.createType("u128", 4),
          amount: api.createType("u128", 1100000000000000),
          maturity: api.createType("u32", 1)
        }
      };
      const {
        data: [result]
      } = await txBondedFinanceOfferSuccessTest(api, walletAlice, requestParameters).catch(error => {
        expect(error.message).to.contain("");
        return error;
      });

      expect(result).to.be.an("Error");
    });
  });
});
