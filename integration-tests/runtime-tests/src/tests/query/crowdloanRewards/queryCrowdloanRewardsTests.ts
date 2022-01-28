/* eslint-disable no-trailing-spaces */
import { expect } from 'chai';

/**
 * Contains all Query tests for the pallet:
 * crowdloanRewards
 *
 * ToDo: Add additional checks.
 */
export class QueryCrowdloanRewardsTests {
  public static runQueryCrowdloanRewardsTests() {
    describe('query.crowdloanRewards.account Tests', function() {
      // Set timeout to 1 minute.
      this.timeout(60*1000);
      it('query.crowdloanRewards.claimedRewards Tests', async function() {
        await QueryCrowdloanRewardsTests.queryCrowdloanRewardsClaimedRewardsTest();
      });

      it('query.crowdloanRewards.totalContributors Tests', async function() {
        await QueryCrowdloanRewardsTests.queryCrowdloanRewardsTotalContributorsTest();
      });

      it('query.crowdloanRewards.totalRewards Tests', async function() {
        await QueryCrowdloanRewardsTests.queryCrowdloanRewardsTotalRewardsTest();
      });
    });
  }

  /**
   * Checks for a successful return of
   * query.crowdloanRewards.claimedRewards()
   */
  private static async queryCrowdloanRewardsClaimedRewardsTest() {
    const claimedRewards = await api.query.crowdloanRewards.claimedRewards();
    expect(claimedRewards.toNumber()).to.be.a('number');
  }

  /**
   * Checks for a successful return of
   * query.crowdloanRewards.totalContributors()
   */
  private static async queryCrowdloanRewardsTotalContributorsTest() {
    const totalContributors = await api.query.crowdloanRewards.totalContributors();
    expect(totalContributors.toNumber()).to.be.a('number');
  }

  /**
   * Checks for a successful return of
   * query.crowdloanRewards.totalRewards()
   */
  private static async queryCrowdloanRewardsTotalRewardsTest() {
    const totalRewards = await api.query.crowdloanRewards.totalRewards();
    expect(totalRewards.toNumber()).to.be.a('number');
  }
}

// Uncomment to debug
// QueryCrowdloanRewardsTests.runQueryCrowdloanRewardsTests();
