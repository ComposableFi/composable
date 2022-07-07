import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { Option, u128, u64, VecAny } from "@polkadot/types-codec";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { Moment } from "@polkadot/types/interfaces/runtime";
import BN from "bn.js";
import { expect } from "chai";

/**
 * Airdrop Tests
 * 1. Create Airdrop
 */
describe.only("tx.airdrop Tests", function() {
  if (!testConfiguration.enabledTests.query.enabled) return;

  let api: ApiPromise;
  let airdrop1Maintainer: KeyringPair,
    airdrop2Maintainer: KeyringPair,
    sudoKey: KeyringPair;
  let DEFAULT_VESTING_PERIOD: Moment;
  let airdrop1_id: BN,
    airdrop2_id: BN;


  before("Setting up the tests", async function() {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    const { devWalletAlice } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    airdrop1Maintainer = devWalletAlice.derive("/tests/airdrop/1/maintainer");
    airdrop2Maintainer = devWalletAlice.derive("/tests/airdrop/2/maintainer");
    DEFAULT_VESTING_PERIOD = api.createType("Moment", 3600 * 24 * 7 * 10);
  });

  before("Providing funds", async function() {
    this.timeout(2 * 60 * 1000);
    await mintAssetsToWallet(api, airdrop1Maintainer, sudoKey, [1]);
  });

  after("Closing the connection", async function() {
    await api.disconnect();
  });

  describe("tx.airdrop.createAirdrop Tests", function() {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    it.only("Any user can create a new AirDrop with defined start", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const airdropCountBeforeCreation = new BN(await api.query.airdrop.airdropCount());

      const startAt: Option<u64> = null;
      const vestingSchedule = api.createType("u64", DEFAULT_VESTING_PERIOD);
      const { data: [result] } = await TxAirdropTests.createAirdrop(api, airdrop1Maintainer, startAt, vestingSchedule);

      const airdropCountAfterCreation = new BN(await api.query.airdrop.airdropCount());
      expect(airdropCountAfterCreation).to.be.bignumber.greaterThan(airdropCountBeforeCreation);
      airdrop1_id = airdropCountAfterCreation;

      const airdropInformation = await api.query.airdrop.airdrops(airdrop1_id);
      expect(airdropInformation.unwrap().creator).to.be.eql(api.createType("AccountId", airdrop1Maintainer.publicKey));
      expect(airdropInformation.unwrap().total_funds).to.be.eql(undefined);
      expect(airdropInformation.unwrap().total_recipients).to.be.eql(undefined);
      expect(airdropInformation.unwrap().start.isNone).to.be.true;
      expect(airdropInformation.unwrap().schedule).to.be.bignumber.equal(api.createType("Moment", DEFAULT_VESTING_PERIOD));
      expect(airdropInformation.unwrap().disabled).to.be.eql(api.createType("bool", false));

    });

    it("Any user can create a new AirDrop with defined start", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const airdropCountBeforeCreation = new BN(await api.query.airdrop.airdropCount());

      const startAt: Option<u64> = null;
      const vestingSchedule = api.createType("u64", DEFAULT_VESTING_PERIOD);
      const { data: [result] } = await TxAirdropTests.createAirdrop(api, airdrop1Maintainer, startAt, vestingSchedule);

      const airdropCountAfterCreation = new BN(await api.query.airdrop.airdropCount());
      expect(airdropCountAfterCreation).to.be.bignumber.greaterThan(airdropCountBeforeCreation);
      airdrop2_id = airdropCountAfterCreation;

      const airdropInformation = await api.query.airdrop.airdrops(airdrop2_id);
      expect(airdropInformation.unwrap().creator).to.be.eql(api.createType("AccountId", airdrop1Maintainer.publicKey));
      expect(airdropInformation.unwrap().total_funds).to.be.eql(undefined);
      expect(airdropInformation.unwrap().total_recipients).to.be.eql(undefined);
      expect(airdropInformation.unwrap().start.isNone).to.be.true;
      expect(airdropInformation.unwrap().schedule).to.be.bignumber.equal(api.createType("Moment", DEFAULT_VESTING_PERIOD));
      expect(airdropInformation.unwrap().disabled).to.be.eql(api.createType("bool", false));
    });
  });

  describe("tx.airdrop.addRecipient Tests", function() {

    it("Airdrop Maintainer can add recipients", async function() {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      const recipientList = api.createType('Vec<(PalletAirdropModelsIdentity, u128, u64, bool)>', [
        {
          // ToDo
        },
      ]);
      const airdropId = api.createType('u128', airdrop1_id);

      const { data: [result] } = await TxAirdropTests.addRecipient(api, airdrop1Maintainer, airdropId, recipientList);

      console.debug(result);

    });
  });
});


export class TxAirdropTests {
  /**
   * ToDo
   *
   * @param {ApiPromise} api Connected API Promise.
   * @param wallet
   * @param startAt
   * @param vestingSchedule
   */
  public static async createAirdrop(api: ApiPromise, wallet: KeyringPair, startAt: Option<u64>, vestingSchedule: u64) {
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.airdrop.AirdropCreated.is,
      api.tx.airdrop.createAirdrop(startAt, vestingSchedule)
    );
  }

  public static async addRecipient(api: ApiPromise, wallet: KeyringPair, airdropId: u128, recipients: VecAny<any>) { // ToDo: Check
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.airdrop.RecipientsAdded.is,
      api.tx.airdrop.addRecipient(airdropId, recipients)
    );
  }
}
