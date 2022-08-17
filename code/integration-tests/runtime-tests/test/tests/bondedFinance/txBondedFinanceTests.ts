import { expect } from "chai";
import testConfiguration from "./test_configuration.json";
import {
  txBondedFinanceOfferFailureTest,
  txBondedFinanceOfferSuccessTest,
  verifyOfferCreation
} from "@composabletests/tests/bondedFinance/testHandlers/offerTests";
import { txBondedFinanceBondSuccessTest } from "@composabletests/tests/bondedFinance/testHandlers/bondTests";
import {
  txBondedFinanceCancelFailureTest,
  txBondedFinanceCancelSuccessTest,
  txBondedFinanceCancelSudoSuccessTest
} from "@composabletests/tests/bondedFinance/testHandlers/cancelTests";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { ApiPromise } from "@polkadot/api";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { KeyringPair } from "@polkadot/keyring/types";
import { u128 } from "@polkadot/types-codec";

/**
 * Contains all TX tests for the pallet:
 * Bonded Finance
 *
 * 1. Create offer
 * 2. Bond to newly created offer
 * 3. Cancel offer by sudo
 */
describe.only("tx.bondedFinance Tests", function () {
  if (!testConfiguration.enabledTests.enabled) return;
  let api: ApiPromise;
  let bondOfferCreatorWallet: KeyringPair, bondOfferBeneficiaryWallet: KeyringPair, bondingWallet: KeyringPair;
  let sudoKey: KeyringPair;
  let bondOfferId1: number, bondOfferId2: number, bondOfferId3: number;

  before("Setting up tests", async function () {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice, devWalletBob } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    bondOfferCreatorWallet = devWalletBob.derive("/test/bondedFinance/offer/creator");
    bondOfferBeneficiaryWallet = devWalletBob.derive("/test/bondedFinance/offer/beneficiary");
    bondingWallet = devWalletAlice.derive("/test/bondedFinance/bonding");
  });

  before("Providing funds for wallets", async function () {
    this.timeout(2 * 60 * 1000);
    await mintAssetsToWallet(api, bondOfferCreatorWallet, sudoKey, [1, 4]);
    await mintAssetsToWallet(api, bondOfferBeneficiaryWallet, sudoKey, [1, 4]);
    await mintAssetsToWallet(api, bondingWallet, sudoKey, [1, 4]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  /**
   * Creating new offers by users.
   */
  describe("tx.bondedFinance.offer Success Tests", function () {
    if (!testConfiguration.enabledTests.offer_bond__success.enabled) return;
    it("[SHORT] User can create new bonded finance offer", async function () {
      if (!testConfiguration.enabledTests.offer_bond__success.create1) this.skip();
      this.timeout(2 * 60 * 1000);
      const bondOfferCountBefore = await api.query.bondedFinance.bondOfferCount();
      const requestParameters = {
        beneficiary: bondOfferBeneficiaryWallet.publicKey,
        asset: api.createType("u128", 1),
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
      } = await txBondedFinanceOfferSuccessTest(api, bondOfferCreatorWallet, requestParameters);
      expect(result.toNumber()).to.be.a("number");
      bondOfferId1 = result.toNumber();
      const bondOfferCountAfter = await api.query.bondedFinance.bondOfferCount();
      expect((<u128>bondOfferCountAfter).toNumber()).to.be.equal((<u128>bondOfferCountBefore).toNumber() + 1);
      await verifyOfferCreation(api, bondOfferId1, requestParameters, bondOfferBeneficiaryWallet);
    });

    it("Can create a second new offer", async function () {
      if (!testConfiguration.enabledTests.offer_bond__success.create2) this.skip();
      this.timeout(2 * 60 * 1000);
      const bondOfferCountBefore = await api.query.bondedFinance.bondOfferCount();
      const requestParameters = {
        beneficiary: bondOfferBeneficiaryWallet.publicKey,
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
      const {
        data: [result]
      } = await txBondedFinanceOfferSuccessTest(api, bondOfferCreatorWallet, requestParameters);
      expect(result.toNumber()).to.be.a("number");
      bondOfferId2 = result.toNumber();
      const bondOfferCountAfter = await api.query.bondedFinance.bondOfferCount();
      expect((<u128>bondOfferCountAfter).toNumber()).to.be.equal((<u128>bondOfferCountBefore).toNumber() + 1);
      await verifyOfferCreation(api, bondOfferId2, requestParameters, bondOfferBeneficiaryWallet);
    });

    it("Can create a third new offer", async function () {
      if (!testConfiguration.enabledTests.offer_bond__success.create2) this.skip();
      this.timeout(2 * 60 * 1000);
      const bondOfferCountBefore = await api.query.bondedFinance.bondOfferCount();
      const requestParameters = {
        beneficiary: bondOfferBeneficiaryWallet.publicKey,
        asset: api.createType("u128", 1),
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
      } = await txBondedFinanceOfferSuccessTest(api, bondOfferCreatorWallet, requestParameters);
      expect(result.toNumber()).to.be.a("number");
      bondOfferId3 = result.toNumber();
      const bondOfferCountAfter = await api.query.bondedFinance.bondOfferCount();
      expect((<u128>bondOfferCountAfter).toNumber()).to.be.equal((<u128>bondOfferCountBefore).toNumber() + 1);
      await verifyOfferCreation(api, bondOfferId3, requestParameters, bondOfferBeneficiaryWallet);
    });
  });

  /**
   * Runs all tx FAILURE tests for the bondedFinance pallet.
   */
  describe("tx.bondedFinance.offer Failure Tests", function () {
    if (!testConfiguration.enabledTests.offer_bond__failure.enabled) return;

    it("Should not be able to create offer (bondPrice < MIN_VESTED_TRANSFER)", async function () {
      if (!testConfiguration.enabledTests.offer_bond__failure.create_offer_price_lt_MIN_VESTED_TRANSFER) this.skip();
      this.timeout(2 * 60 * 1000);
      const bondOfferCountBefore = await api.query.bondedFinance.bondOfferCount();
      const requestParameters = {
        beneficiary: bondOfferBeneficiaryWallet.publicKey,
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
      await txBondedFinanceOfferFailureTest(api, bondOfferCreatorWallet, requestParameters).catch(error => {
        expect(error.toString()).to.contain("RpcError: 1002: Verification Error");
      });
      const bondOfferCountAfter = await api.query.bondedFinance.bondOfferCount();
      expect((<u128>bondOfferCountAfter).toNumber()).to.be.equal((<u128>bondOfferCountBefore).toNumber());
    });

    it("Should not be able to create offer (reward.amount < MinReward)", async function () {
      if (!testConfiguration.enabledTests.offer_bond__failure.create_offer_reward_amt_lt_MinReward) this.skip();
      this.timeout(2 * 60 * 1000);
      const bondOfferCountBefore = await api.query.bondedFinance.bondOfferCount();
      const requestParameters = {
        beneficiary: bondOfferBeneficiaryWallet.publicKey,
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
      await txBondedFinanceOfferFailureTest(api, bondOfferCreatorWallet, requestParameters).catch(error => {
        expect(error.toString()).to.contain("RpcError: 1002: Verification Error");
      });
      const bondOfferCountAfter = await api.query.bondedFinance.bondOfferCount();
      expect((<u128>bondOfferCountAfter).toNumber()).to.be.equal((<u128>bondOfferCountBefore).toNumber());
    });

    it("Should not be able to create offer (reward.asset does not exist)", async function () {
      if (!testConfiguration.enabledTests.offer_bond__failure.create_offer_reward_asset_not_exist) this.skip();
      this.timeout(2 * 60 * 1000);
      const bondOfferCountBefore = await api.query.bondedFinance.bondOfferCount();
      const requestParameters = {
        beneficiary: bondOfferBeneficiaryWallet.publicKey,
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
      await txBondedFinanceOfferFailureTest(api, bondOfferCreatorWallet, requestParameters).catch(error => {
        expect(error.toString()).to.contain("RpcError: 1002: Verification Error");
      });
      const bondOfferCountAfter = await api.query.bondedFinance.bondOfferCount();
      expect((<u128>bondOfferCountAfter).toNumber()).to.be.equal((<u128>bondOfferCountBefore).toNumber());
    });
  });

  /**
   * Trying to bond to the newly created offer.
   */
  describe("tx.bondedFinance.bond Success Tests", function () {
    it("[SHORT] Can single bond to newly created offer", async function () {
      if (!testConfiguration.enabledTests.offer_bond__success.bond) this.skip();
      this.timeout(2 * 60 * 1000);
      const offerId = api.createType("u64", bondOfferId1);
      const nbOfBonds = api.createType("u128", 1);
      const {
        data: [result]
      } = await txBondedFinanceBondSuccessTest(api, bondingWallet, offerId, nbOfBonds);
      expect(result.toNumber()).to.be.equal(bondOfferId1);
    });

    it("Can multi bond to newly created offer", async function () {
      if (!testConfiguration.enabledTests.offer_bond__success.bond) this.skip();
      this.timeout(2 * 60 * 1000);
      const offerId = api.createType("u64", bondOfferId1);
      const nbOfBonds = api.createType("u128", 9);
      const {
        data: [result]
      } = await txBondedFinanceBondSuccessTest(api, bondingWallet, offerId, nbOfBonds);
      expect(result.toNumber()).to.be.equal(bondOfferId1);
    });
  });

  describe("tx.bondedFinance.bond Failure Tests", function () {
    it("Can not bond to finished/sold out offer ", async function () {
      if (!testConfiguration.enabledTests.offer_bond__success.bond) this.skip();
      this.timeout(2 * 60 * 1000);
      const offerId = api.createType("u64", bondOfferId1);
      const nbOfBonds = api.createType("u128", 1);
      await txBondedFinanceBondSuccessTest(api, bondingWallet, offerId, nbOfBonds).catch(error => {
        expect(error.toString()).to.contain("bondedFinance.OfferCompleted");
      });
    });
  });

  /**
   * Runs FAILURE tests for bondedFinance.cancel(offerId)
   */
  describe("tx.bondedFinance.cancel Failure Tests", function () {
    if (!testConfiguration.enabledTests.cancel_failure.enabled) return;
    it("Should not be able to cancel offer that doesn't exist", async function () {
      if (!testConfiguration.enabledTests.cancel_failure.cancel_offer_not_exist) this.skip();
      this.timeout(2 * 60 * 1000);
      const offerId = 1337;
      const {
        data: [result]
      } = await txBondedFinanceCancelFailureTest(api, bondOfferBeneficiaryWallet, offerId);
      expect(result.toString()).to.be.equal('{"module":{"index":60,"error":"0x00000000"}}');
    });

    it("Should not be able to cancel offer by someone else", async function () {
      if (!testConfiguration.enabledTests.cancel_failure.cancel_offer_not_exist) this.skip();
      this.timeout(2 * 60 * 1000);
      const {
        data: [result]
      } = await txBondedFinanceCancelFailureTest(api, bondingWallet, bondOfferId1);
      expect(result.toString()).to.be.equal("BadOrigin");
    });

    it("Creator can not cancel offer", async function () {
      if (!testConfiguration.enabledTests.cancel_success.cancel_by_creator) this.skip();
      this.timeout(2 * 60 * 1000);
      await mintAssetsToWallet(api, bondOfferCreatorWallet, sudoKey, [1, 4]);
      console.debug();
      const {
        data: [result]
      } = await txBondedFinanceCancelSuccessTest(api, bondOfferCreatorWallet, bondOfferId3);
      expect(result).to.be.an("Error");
      expect(result.toString()).to.contain("BadOrigin");
    });
  });

  /**
   * Cancelling the 2 offers created in the previous tests.
   */
  describe("tx.bondedFinance.cancel Success Tests", function () {
    if (!testConfiguration.enabledTests.cancel_success.enabled) return;

    it("Sudo cancel second offer", async function () {
      if (!testConfiguration.enabledTests.cancel_success.cancel_by_sudo) this.skip();
      this.timeout(2 * 60 * 1000);
      const {
        data: [result, result2, result3]
      } = await txBondedFinanceCancelSudoSuccessTest(api, sudoKey, [bondOfferId1, bondOfferId2, bondOfferId3]);
      expect(result.isOk).to.be.true;
      expect(result2.isOk).to.be.true;
      expect(result3.isOk).to.be.true;
      const offer1Info = await api.query.bondedFinance.bondOffers(bondOfferId1);
      const offer2Info = await api.query.bondedFinance.bondOffers(bondOfferId2);
      const offer3Info = await api.query.bondedFinance.bondOffers(bondOfferId3);
      expect(offer1Info.unwrapOr(null)).to.be.null;
      expect(offer2Info.unwrapOr(null)).to.be.null;
      expect(offer3Info.unwrapOr(null)).to.be.null;
      console.debug(offer1Info.toString());
      console.debug(offer2Info.toString());
      console.debug(offer3Info.toString());
    });
  });
});
