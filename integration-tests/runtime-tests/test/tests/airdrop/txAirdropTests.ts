import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { Option, u64 } from "@polkadot/types-codec";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { Moment } from "@polkadot/types/interfaces/runtime";
import BN from "bn.js";
import { expect } from "chai";
import { PalletAirdropModelsIdentity, PalletAirdropModelsProof } from "@composable/types/interfaces";
import { ethAccount, proofMessage, TxAirdropTests } from "@composabletests/tests/airdrop/airdropTestHandler";

/**
 * Airdrop Tests
 * 1. Create Airdrop
 */
describe.only("tx.airdrop Tests", function() {
  if (!testConfiguration.enabledTests.query.enabled) return;

  let api: ApiPromise;
  let airdrop1Maintainer: KeyringPair,
    airdrop2Maintainer: KeyringPair,
    airdrop1Recipient2RelayChain: KeyringPair,
    airdrop1Recipient1: KeyringPair,
    airdrop1Recipient3Eth: KeyringPair,
    airdrop1Recipient4Cosmos: KeyringPair,
    airdrop2Recipient1: KeyringPair,
    airdrop2Recipient2: KeyringPair,
    sudoKey: KeyringPair;
  let DEFAULT_VESTING_PERIOD: Moment;
  let airdrop1_id: BN,
    airdrop2_id: BN;


  before("Setting up the tests", async function() {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    const { devWalletAlice, devWalletBob } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    airdrop1Maintainer = devWalletAlice.derive("/tests/airdrop/1/maintainer");
    airdrop2Maintainer = devWalletBob.derive("/tests/airdrop/2/maintainer");

    airdrop1Recipient1 = devWalletAlice.derive("/tests/airdrop/1/recipient/1");
    airdrop1Recipient2RelayChain = devWalletAlice.derive("/tests/airdrop/2/recipient/1");
    airdrop1Recipient3Eth = devWalletAlice.derive("/tests/airdrop/1/recipient/3");
    airdrop1Recipient4Cosmos = devWalletAlice.derive("/tests/airdrop/1/recipient/4");
    airdrop2Recipient1 = devWalletBob.derive("/tests/airdrop/1/recipient/1");
    airdrop2Recipient2 = devWalletBob.derive("/tests/airdrop/1/recipient/2");

    DEFAULT_VESTING_PERIOD = api.createType("Moment", 3600 * 24 * 7 * 10);
  });

  before("Providing funds", async function() {
    this.timeout(2 * 60 * 1000);
    await mintAssetsToWallet(api, airdrop1Maintainer, sudoKey, [1]);
    await mintAssetsToWallet(api, airdrop2Maintainer, sudoKey, [1]);
    await mintAssetsToWallet(api, airdrop1Recipient2RelayChain, sudoKey, [1]);
  });

  after("Closing the connection", async function() {
    await api.disconnect();
  });

  describe("tx.airdrop.createAirdrop Tests", function() {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    it("Any user can create a new AirDrop with defined start", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const airdropCountBeforeCreation = new BN(await api.query.airdrop.airdropCount());
      const currentBlock = await api.query.system.number();

      const startAt: Option<u64> = null;
      const vestingSchedule = api.createType("u64", currentBlock.add(new BN(16)));

      const { data: [result] } = await TxAirdropTests.createAirdrop(api, airdrop1Maintainer, startAt, vestingSchedule);
      // ToDo: Result check!

      const airdropCountAfterCreation = new BN(await api.query.airdrop.airdropCount());
      expect(airdropCountAfterCreation).to.be.bignumber.greaterThan(airdropCountBeforeCreation);
      airdrop1_id = airdropCountAfterCreation;

      await TxAirdropTests.verifyAirdropCreation(api, airdrop1_id, airdrop1Maintainer.publicKey, startAt, vestingSchedule);
    });

    it("Any user can create a new AirDrop with defined start", async function() {
      this.skip();
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const airdropCountBeforeCreation = new BN(await api.query.airdrop.airdropCount());
      const currentBlock = await api.query.system.number();

      const startAt = api.createType("Option<u64>", currentBlock.add(new BN(500)));
      const vestingSchedule = api.createType("u64", DEFAULT_VESTING_PERIOD);//api.createType("u64", currentBlock.add(new BN(1600)));
      const { data: [result] } = await TxAirdropTests.createAirdrop(api, airdrop2Maintainer, startAt, vestingSchedule);
      // ToDo: Result check!

      const airdropCountAfterCreation = new BN(await api.query.airdrop.airdropCount());
      expect(airdropCountAfterCreation).to.be.bignumber.greaterThan(airdropCountBeforeCreation);
      airdrop2_id = airdropCountAfterCreation;

      await TxAirdropTests.verifyAirdropCreation(api, airdrop2_id, airdrop2Maintainer.publicKey, startAt, vestingSchedule);
    });
  });

  describe("tx.airdrop.addRecipient Tests", function() {

    it("Airdrop [#1] Maintainer can add recipients", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const recipientList = api.createType("Vec<(PalletAirdropModelsIdentity, u128, u64, bool)>", [
        {
          IdentityOf: { RelayChain: api.createType("AccountId32", airdrop1Recipient1.publicKey) },
          BalanceOf: api.createType("u128", 100000000000),
          MomentOf: api.createType("u64", 0),
          bool: api.createType("bool", true)
        },
        {
          IdentityOf: { RelayChain: api.createType("AccountId32", airdrop1Recipient2RelayChain.publicKey) },
          BalanceOf: api.createType("u128", 100000000000),
          MomentOf: api.createType("u64", 0),
          bool: api.createType("bool", true)
        },
        {
          IdentityOf: {
            Ethereum: api.createType("ComposableSupportEthereumAddress",
              ethAccount(1).address)
          },
          BalanceOf: api.createType("u128", 100000000000),
          MomentOf: api.createType("u64", 0),
          bool: api.createType("bool", true)
        },
        {
          // ToDo: Add Cosmos support
          IdentityOf: { RelayChain: api.createType("AccountId32", airdrop1Recipient4Cosmos.publicKey) },
          BalanceOf: api.createType("u128", 100000000000),
          MomentOf: api.createType("u64", 0),
          bool: api.createType("bool", true)
        }
      ]);
      const airdropId = api.createType("u128", airdrop1_id);

      const { data: [result] } = await TxAirdropTests.addRecipient(api, airdrop1Maintainer, airdropId, recipientList);
      // ToDo: Result check!


      console.debug(result);

    });

    it("Airdrop [#2] Maintainer can add recipients", async function() {
      this.skip();
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const recipientList = api.createType("Vec<(PalletAirdropModelsIdentity, u128, u64, bool)>", [
        {
          IdentityOf: { RelayChain: api.createType("AccountId32", airdrop2Recipient1.publicKey) },
          BalanceOf: api.createType("u128", 100000000000),
          MomentOf: api.createType("u64", 0),
          bool: api.createType("bool", false)
        },
        {
          IdentityOf: { RelayChain: api.createType("AccountId32", airdrop2Recipient2.publicKey) },
          BalanceOf: api.createType("u128", 100000000000),
          MomentOf: api.createType("u64", 0),
          bool: api.createType("bool", false)
        }
      ]);
      const airdropId = api.createType("u128", airdrop2_id);

      const { data: [result] } = await TxAirdropTests.addRecipient(api, airdrop1Maintainer, airdropId, recipientList);

      console.debug(result);

    });
  });

  describe("tx.airdrop.removeRecipient Tests", function() {

    it("Airdrop [#1] Maintainer can remove recipients", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const recipient = api.createType("PalletAirdropModelsIdentity", {
        RelayChain: api.createType("AccountId32", airdrop1Recipient1.publicKey)
      });
      const airdropId = api.createType("u128", airdrop1_id);

      const { data: [result] } = await TxAirdropTests.removeRecipient(api, airdrop1Maintainer, airdropId, recipient);
      // ToDo: Result check!


      console.debug(result);

    });

    it("Airdrop [#2] Maintainer can remove recipients", async function() {
      this.skip();
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const recipient = api.createType("PalletAirdropModelsIdentity", {
        RelayChain: api.createType("AccountId32", airdrop2Recipient2.publicKey)
      });
      const airdropId = api.createType("u128", airdrop2_id);

      const { data: [result] } = await TxAirdropTests.removeRecipient(api, airdrop1Maintainer, airdropId, recipient);

      console.debug(result);

    });
  });

  describe("tx.airdrop.claim Failure Tests", function() {
    it("Airdrop [#1] can not be claimed before the start", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const airdropId = api.createType("u128", airdrop1_id);
      const rewardAccount = api.createType("AccountId32", airdrop1Recipient2RelayChain.publicKey);
      const proof = api.createType("PalletAirdropModelsProof", {
        RelayChain:
          api.createType("(AccountId32, SpRuntimeMultiSignature)", [
            api.createType("AccountId32", airdrop1Recipient2RelayChain.publicKey),
            api.createType("SpRuntimeMultiSignature", {
              Sr25519: api.createType("SpCoreSr25519Signature", airdrop1Recipient2RelayChain.sign(proofMessage(airdrop1Recipient2RelayChain)))
            })
          ])
      });

      const { data: [result] } = await TxAirdropTests.claimAirdrop(
        api,
        airdropId,
        rewardAccount,
        proof
      ).catch(function(error) {
        // ToDo: Check!
        expect(error.message).to.contain("Custom error: 3");
        return { data: [error] };
      });
    });
  });

  describe("tx.airdrop.enableAirdrop Tests", function() {
    it("Airdrop [#1] can be enabled by maintainer", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const recipient = api.createType("PalletAirdropModelsIdentity", {
        RelayChain: api.createType("AccountId32", airdrop1Recipient1.publicKey)
      });
      const airdropId = api.createType("u128", airdrop1_id);

      const { data: [result] } = await TxAirdropTests.enableAirdrop(api, airdrop1Maintainer, airdropId);
      // ToDo: Result check!


      console.debug(result);

    });

    it("Airdrop [#2] can be enabled by maintainer", async function() {
      this.skip();
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const recipient = api.createType("PalletAirdropModelsIdentity",
        {
          RelayChain: api.createType("AccountId32", airdrop2Recipient2.publicKey)
        });
      const airdropId = api.createType("u128", airdrop2_id);

      const { data: [result] } = await TxAirdropTests.enableAirdrop(api, airdrop1Maintainer, airdropId);

      console.debug(result);

    });
  });

  describe("tx.airdrop.claim Tests", function() {
    it("Airdrop [#1] can be claimed by RelayChain contributor with correct proof", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const airdropId = api.createType("u128", airdrop1_id);
      const rewardAccount = api.createType("AccountId32", airdrop1Recipient2RelayChain.publicKey);
      const proof = api.createType("PalletAirdropModelsProof", {
        RelayChain:
          api.createType("(AccountId32, SpRuntimeMultiSignature)", [
            api.createType("AccountId32", airdrop1Recipient2RelayChain.publicKey),
            api.createType("SpRuntimeMultiSignature", {
              Sr25519: api.createType("SpCoreSr25519Signature", airdrop1Recipient2RelayChain.sign(proofMessage(airdrop1Recipient2RelayChain)))
            })
          ])
      });

      const { data: [result] } = await TxAirdropTests.claimAirdrop(
        api,
        airdropId,
        rewardAccount,
        proof
      );
      // ToDo: Result check!

      console.debug(result);
    });

    it("Airdrop [#1] can be claimed by Ethereum contributor with correct proof", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const airdropId = api.createType("u128", airdrop1_id);
      const rewardAccount = api.createType("AccountId32", airdrop1Recipient3Eth.publicKey);
      const proofSignature: any = airdrop1Recipient3Eth.sign(proofMessage(airdrop1Recipient3Eth, true));
      const proof = api.createType("PalletAirdropModelsProof", {
        Ethereum: api.createType("ComposableSupportEcdsaSignature", proofSignature.signature)
      });

      const { data: [result] } = await TxAirdropTests.claimAirdrop(
        api,
        airdropId,
        rewardAccount,
        proof
      );
      // ToDo: Result check!

      console.debug(result);
    });

    it("Airdrop [#1] can not be claimed by removed contributor with correct proof", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const airdropId = api.createType("u128", airdrop1_id);
      const rewardAccount = api.createType("AccountId32", airdrop1Recipient1.publicKey);
      const proof = api.createType("PalletAirdropModelsProof", {
        RelayChain:
          api.createType("(AccountId32, SpRuntimeMultiSignature)", [
            api.createType("AccountId32", airdrop1Recipient1.publicKey),
            api.createType("SpRuntimeMultiSignature", {
              Sr25519: api.createType("SpCoreSr25519Signature", airdrop1Recipient1.sign(proofMessage(airdrop1Recipient1)))
            })
          ])
      });

      const { data: [result] } = await TxAirdropTests.claimAirdrop(
        api,
        airdropId,
        rewardAccount,
        proof
      );
      // ToDo: Result check!

      console.debug(result);
    });

    it("Airdrop [#2] can be claimed with correct proof", async function() {
      this.skip();
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const recipient = api.createType("PalletAirdropModelsIdentity", {
        RelayChain: api.createType("AccountId32", airdrop2Recipient2.publicKey)
      });
      const airdropId = api.createType("u128", airdrop2_id);

      const { data: [result] } = await TxAirdropTests.disableAirdrop(api, airdrop1Maintainer, airdropId);

      console.debug(result);

    });
  });

  describe("tx.airdrop.disableAirdrop Tests", function() {
    it("Airdrop [#1] can be disabled by maintainer", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const recipient = api.createType("PalletAirdropModelsIdentity",
        {
          RelayChain: api.createType("AccountId32", airdrop1Recipient1.publicKey)
        });
      const airdropId = api.createType("u128", airdrop1_id);

      const { data: [result] } = await TxAirdropTests.disableAirdrop(api, airdrop1Maintainer, airdropId);
      // ToDo: Result check!


      console.debug(result);

    });

    it("Airdrop [#2] can be disabled by maintainer", async function() {
      this.skip();
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const recipient = api.createType("PalletAirdropModelsIdentity",
        {
          RelayChain: api.createType("AccountId32", airdrop2Recipient2.publicKey)
        });
      const airdropId = api.createType("u128", airdrop2_id);

      const { data: [result] } = await TxAirdropTests.disableAirdrop(api, airdrop1Maintainer, airdropId);

      console.debug(result);

    });
  });
});
