import { expect } from "chai";
import testConfiguration from "./test_configuration.json";
import {
  txBondedFinanceOfferFailureTest,
  txBondedFinanceOfferSuccessTest
} from "@composabletests/tests/bondedFinance/testHandlers/offerTests";
import { txBondedFinanceBondSuccessTest } from "@composabletests/tests/bondedFinance/testHandlers/bondTests";
import {
  txBondedFinanceCancelFailureTest,
  txBondedFinanceCancelSuccessTest, txBondedFinanceCancelSudoSuccessTest
} from "@composabletests/tests/bondedFinance/testHandlers/cancelTests";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";


/**
 * Contains all TX tests for the pallet:
 * bondedFinance
 */
describe("tx.bondedFinance Tests", function() {
  if (!testConfiguration.enabledTests.enabled)
    return;

  let bondOfferId1: number, bondOfferId2: number;
  /**
   * bondedFinance.offer(...) Success Tests
   */
  describe("tx.bondedFinance.offer & .bond Success Tests", function() {
    if (!testConfiguration.enabledTests.offer_bond__success.enabled)
      return;

    // Timeout set to 2 minutes
    this.timeout(2 * 60 * 1000);
    // #1 Create offer using Alice's wallet.
    before("mint assets into the wallet", async function() {
      this.timeout(2 * 60 * 1000);
      await mintAssetsToWallet(walletAlice, walletAlice, [4]);
      await mintAssetsToWallet(walletBob, walletAlice, [4]);
    });
    it("Can create a new offer", async function() {
      if (!testConfiguration.enabledTests.offer_bond__success.create1)
        this.skip();
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
      const { data: [result] } = await txBondedFinanceOfferSuccessTest(walletAlice, requestParameters);
      expect(result.toNumber()).to.be.a("number");
      bondOfferId1 = result.toNumber();
    });

    // #2 Create offer using Bob's wallet.
    it("Can create a second new offer", async function() {
      if (!testConfiguration.enabledTests.offer_bond__success.create2)
        this.skip();
      const requestParameters = {
        beneficiary: walletBob.publicKey,
        asset: api.createType("u128", 4),
        bondPrice: api.createType("u128", 100000000000000),
        nbOfBonds: api.createType("u128", 10),
        maturity: { Finite: { returnIn: api.createType("u32", 16) } },
        reward: {
          asset: api.createType("u128", 1),
          amount: api.createType("u128", 1100000000000000),
          maturity: api.createType("u32", 1)
        }
      };
      const { data: [result] } = await txBondedFinanceOfferSuccessTest(walletBob, requestParameters);
      expect(result.toNumber()).to.be.a("number");
      bondOfferId2 = result.toNumber();
    });

    /**
     * bondedFinance.bond(offerId:u64, nbOfBonds:u128) Tests
     */
    // #3 Bob can bond to the offer Alice has created.
    it("Can bond to newly created offer", async function() {
      if (!testConfiguration.enabledTests.offer_bond__success.bond)
        this.skip();
      const offerId = api.createType("u64", bondOfferId1);
      const nbOfBonds = api.createType("u128", 1);
      const { data: [result] } = await txBondedFinanceBondSuccessTest(walletBob, offerId, nbOfBonds);
      expect(result.toNumber()).to.be
        .equal(bondOfferId1);
    });
  });

  /**
   * Runs all tx FAILURE tests for the bondedFinance pallet.
   */
  describe("tx.bondedFinance.offer Failure Tests", function() {
    if (!testConfiguration.enabledTests.offer_bond__failure.enabled)
      return;
    // Timeout set to 2 minutes
    this.timeout(2 * 60 * 1000);
    // #4 Alice can't create am offer with the bond price too low.
    it("Should not be able to create offer (bondPrice < MIN_VESTED_TRANSFER)", async function() {
      if (!testConfiguration.enabledTests.offer_bond__failure.create_offer_price_lt_MIN_VESTED_TRANSFER)
        this.skip();
      const requestParameters = {
        beneficiary: walletAlice.publicKey,
        asset: api.createType("u128", 4),
        bondPrice: api.createType("u128", api.consts.bondedFinance.stake.toNumber() - 1),
        nbOfBonds: api.createType("u128", 10),
        maturity: { Finite: { returnIn: api.createType("u32", 16) } },
        reward: {
          asset: api.createType("u128", 4),
          amount: api.consts.bondedFinance.minReward,
          maturity: api.createType("u32", 1)
        }
      };
      await txBondedFinanceOfferFailureTest(walletAlice, requestParameters).catch(e =>
        expect(e).to.be.an("Error"));
    });

    // #5 Alice can't create offer with the reward amount too low.
    it("Should not be able to create offer (reward.amount < MinReward)", async function() {
      if (!testConfiguration.enabledTests.offer_bond__failure.create_offer_reward_amt_lt_MinReward)
        this.skip();
      const requestParameters = {
        beneficiary: walletAlice.publicKey,
        asset: api.createType("u128", 4),
        bondPrice: api.consts.bondedFinance.stake,
        nbOfBonds: api.createType("u128", 10),
        maturity: { Finite: { returnIn: api.createType("u32", 16) } },
        reward: {
          asset: api.createType("u128", 4),
          amount: api.createType("u128", api.consts.bondedFinance.minReward.toNumber() - 1),
          maturity: api.createType("u32", 1)
        }
      };
      await txBondedFinanceOfferFailureTest(walletAlice, requestParameters).catch(e =>
        expect(e).to.be.an("Error"));
    });

    // #5 Alice can't create offer with the reward amount too low.
    it("Should not be able to create offer (reward.asset does not exist)", async function() {
      if (!testConfiguration.enabledTests.offer_bond__failure.create_offer_reward_asset_not_exist)
        this.skip();
      const requestParameters = {
        beneficiary: walletAlice.publicKey,
        asset: api.createType("u128", 4),
        bondPrice: api.consts.bondedFinance.stake,
        nbOfBonds: api.createType("u128", 10),
        maturity: { Finite: { returnIn: api.createType("u32", 16) } },
        reward: {
          asset: api.createType("u128", 1337),
          amount: api.consts.bondedFinance.minReward,
          maturity: api.createType("u32", 1)
        }
      };
      await txBondedFinanceOfferFailureTest(walletAlice, requestParameters).catch(e =>
        expect(e).to.be.an("Error"));
    });
  });

  /**
   * Runs FAILURE tests for bondedFinance.cancel(offerId)
   */
  describe("tx.bondedFinance.cancel Failure Tests", function() {
    if (!testConfiguration.enabledTests.cancel_failure.enabled)
      return;
    // Timeout set to 2 minutes
    this.timeout(2 * 60 * 1000);
    it("Should not be able to cancel offer that doesn't exist", async function() {
      if (!testConfiguration.enabledTests.cancel_failure.cancel_offer_not_exist)
        this.skip();
      const offerId = 1337;
      await txBondedFinanceCancelFailureTest(walletAlice, offerId).catch(e =>
        expect(e).to.be.an("Error"));
    });
  });

  /**
   * Runs SUCCESS tests for bondedFinance.cancel(offerId)
   */
  describe("tx.bondedFinance.cancel Success Tests", function() {
    if (!testConfiguration.enabledTests.cancel_success.enabled)
      return;
    // Timeout set to 2 minutes
    this.timeout(2 * 60 * 1000);
    // #6 Alice should be able to cancel her offer.
    it("Can cancel offer created in first bondedFinance.offer test by creator", async function() {
      if (!testConfiguration.enabledTests.cancel_success.cancel_by_creator)
        this.skip();
      const offerId = bondOfferId1;
      const { data: [result] } = await txBondedFinanceCancelSuccessTest(walletAlice, offerId);
      expect(result.toNumber()).to.be.a("number");
      expect(result.toNumber()).to.be
        .equal(bondOfferId1);
    });

    // #7 A sudo command should be able to cancel an offer.
    it("Can sudo (diff. account) cancel offer created in second bondedFinance.offer", async function() {
      if (!testConfiguration.enabledTests.cancel_success.cancel_by_sudo)
        this.skip();
      const offerId = bondOfferId2;
      const { data: [result] } = await txBondedFinanceCancelSudoSuccessTest(walletAlice, offerId);
      expect(result.isOk).to.be.true;
    });
  });
});
