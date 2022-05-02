import { expect } from "chai";
import testConfiguration from "./test_configuration.json";

describe("rpc.crowdloanRewards Tests", function() {
  if (!testConfiguration.enabledTests.rpc.enabled)
    return;
  it("rpc.crowdloanRewards.account Tests", async function() {
    if (!testConfiguration.enabledTests.rpc.account__success)
      this.skip();
    const accountId = walletAlice.derive("/contributor-1/reward").publicKey;
    const result = await RpcCrowdloanRewardsTests.rpcCrowdloanRewardsTest(accountId);
    expect(result).to.be.a["bignumber"].that.equals("0");
  });
});

export class RpcCrowdloanRewardsTests {
  public static async rpcCrowdloanRewardsTest(accountId: string | Uint8Array) {
    return await api.rpc.crowdloanRewards.amountAvailableToClaimFor(
      accountId
    );
  }
}
