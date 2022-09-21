import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import { getNewConnection } from "@composable/utils/connectionHelper";
import testConfiguration from "@composabletests/tests/crowdloanRewards/test_configuration.json";

describe("query.crowdloanRewards.account Tests", function () {
  if (!testConfiguration.enabledTests.query.enabled) return;
  // Set timeout to 1 minute.
  this.timeout(60 * 1000);

  let api: ApiPromise;

  before("Setting up tests", async function () {
    const { newClient } = await getNewConnection();
    api = newClient;
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  it("query.crowdloanRewards.claimedRewards Tests", async function () {
    if (!testConfiguration.enabledTests.query.claimedRewards_success) this.skip();
    const claimedRewards = await QueryCrowdloanRewardsTests.queryCrowdloanRewardsClaimedRewardsTest(api);
    expect(claimedRewards.toNumber()).to.be.a("number");
  });

  it("query.crowdloanRewards.totalContributors Tests", async function () {
    if (!testConfiguration.enabledTests.query.totalContributors_success) this.skip();
    const totalContributors = await QueryCrowdloanRewardsTests.queryCrowdloanRewardsTotalContributorsTest(api);
    expect(totalContributors.toNumber()).to.be.a("number");
  });

  it("query.crowdloanRewards.totalRewards Tests", async function () {
    if (!testConfiguration.enabledTests.query.totalRewards_success) this.skip();
    const totalRewards = await QueryCrowdloanRewardsTests.queryCrowdloanRewardsTotalRewardsTest(api);
    expect(totalRewards.toNumber()).to.be.a("number");
  });
});

/**
 * Contains all Query tests for the pallet:
 * crowdloanRewards
 */
class QueryCrowdloanRewardsTests {
  /**
   * Checks for a successful return of
   * query.crowdloanRewards.claimedRewards()
   */
  public static async queryCrowdloanRewardsClaimedRewardsTest(api: ApiPromise) {
    return await api.query.crowdloanRewards.claimedRewards();
  }

  /**
   * Checks for a successful return of
   * query.crowdloanRewards.totalContributors()
   */
  public static async queryCrowdloanRewardsTotalContributorsTest(api: ApiPromise) {
    return await api.query.crowdloanRewards.totalContributors();
  }

  /**
   * Checks for a successful return of
   * query.crowdloanRewards.totalRewards()
   */
  public static async queryCrowdloanRewardsTotalRewardsTest(api: ApiPromise) {
    return await api.query.crowdloanRewards.totalRewards();
  }
}
