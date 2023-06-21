import { expect } from "chai";
import testConfiguration from "./test_configuration.json";
import { ApiPromise } from "@polkadot/api";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { KeyringPair } from "@polkadot/keyring/types";
import { getDevWallets } from "@composable/utils/walletHelper";

describe("rpc.crowdloanRewards Tests", function () {
  if (!testConfiguration.enabledTests.rpc.enabled) return;
  // Set timeout to 1 minute.
  this.timeout(60 * 1000);

  let api: ApiPromise;
  let walletAlice: KeyringPair;
  before("Setting up tests", async function () {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice } = getDevWallets(newKeyring);
    walletAlice = devWalletAlice;
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  it("rpc.crowdloanRewards.account Tests", async function () {
    if (!testConfiguration.enabledTests.rpc.account__success) this.skip();
    const accountId = walletAlice.derive("/contributor-1/reward").publicKey;
    const result = await RpcCrowdloanRewardsTests.rpcCrowdloanRewardsTest(api, accountId);
    expect(result).to.be.a["bignumber"].that.equals("0");
  });
});

class RpcCrowdloanRewardsTests {
  public static async rpcCrowdloanRewardsTest(api: ApiPromise, accountId: string | Uint8Array) {
    return await api.rpc.crowdloanRewards.amountAvailableToClaimFor(accountId);
  }
}
