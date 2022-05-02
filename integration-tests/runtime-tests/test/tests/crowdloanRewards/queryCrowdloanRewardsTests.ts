import { expect } from "chai";

describe("query.crowdloanRewards.account Tests", function() {
  // Set timeout to 1 minute.
  this.timeout(60 * 1000);
  it("query.crowdloanRewards.claimedRewards Tests", async function() {
    const claimedRewards = await QueryCrowdloanRewardsTests.queryCrowdloanRewardsClaimedRewardsTest();
    expect(claimedRewards.toNumber()).to.be.a("number");
  });

  it("query.crowdloanRewards.totalContributors Tests", async function() {
    const totalContributors = await QueryCrowdloanRewardsTests.queryCrowdloanRewardsTotalContributorsTest();
    expect(totalContributors.toNumber()).to.be.a("number");
  });

  it("query.crowdloanRewards.totalRewards Tests", async function() {
    const totalRewards = await QueryCrowdloanRewardsTests.queryCrowdloanRewardsTotalRewardsTest();
    expect(totalRewards.toNumber()).to.be.a("number");
  });
});

/**
 * Contains all Query tests for the pallet:
 * crowdloanRewards
 */
export class QueryCrowdloanRewardsTests {
  /**
   * Checks for a successful return of
   * query.crowdloanRewards.claimedRewards()
   */
  public static async queryCrowdloanRewardsClaimedRewardsTest() {
    return await api.query.crowdloanRewards.claimedRewards();
  }

  /**
   * Checks for a successful return of
   * query.crowdloanRewards.totalContributors()
   */
  public static async queryCrowdloanRewardsTotalContributorsTest() {
    return await api.query.crowdloanRewards.totalContributors();
  }

  /**
   * Checks for a successful return of
   * query.crowdloanRewards.totalRewards()
   */
  public static async queryCrowdloanRewardsTotalRewardsTest() {
    return await api.query.crowdloanRewards.totalRewards();
  }
}
