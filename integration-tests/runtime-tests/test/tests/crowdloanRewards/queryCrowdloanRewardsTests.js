"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.QueryCrowdloanRewardsTests = void 0;
/* eslint-disable no-trailing-spaces */
const chai_1 = require("chai");
describe('query.crowdloanRewards.account Tests', function () {
    // Set timeout to 1 minute.
    this.timeout(60 * 1000);
    it('query.crowdloanRewards.claimedRewards Tests', async function () {
        await QueryCrowdloanRewardsTests.queryCrowdloanRewardsClaimedRewardsTest();
    });
    it('query.crowdloanRewards.totalContributors Tests', async function () {
        await QueryCrowdloanRewardsTests.queryCrowdloanRewardsTotalContributorsTest();
    });
    it('query.crowdloanRewards.totalRewards Tests', async function () {
        await QueryCrowdloanRewardsTests.queryCrowdloanRewardsTotalRewardsTest();
    });
});
/**
 * Contains all Query tests for the pallet:
 * crowdloanRewards
 *
 * ToDo: Add additional checks.
 */
class QueryCrowdloanRewardsTests {
    /**
     * Checks for a successful return of
     * query.crowdloanRewards.claimedRewards()
     */
    static async queryCrowdloanRewardsClaimedRewardsTest() {
        const claimedRewards = await api.query.crowdloanRewards.claimedRewards();
        (0, chai_1.expect)(claimedRewards.toNumber()).to.be.a('number');
    }
    /**
     * Checks for a successful return of
     * query.crowdloanRewards.totalContributors()
     */
    static async queryCrowdloanRewardsTotalContributorsTest() {
        const totalContributors = await api.query.crowdloanRewards.totalContributors();
        (0, chai_1.expect)(totalContributors.toNumber()).to.be.a('number');
    }
    /**
     * Checks for a successful return of
     * query.crowdloanRewards.totalRewards()
     */
    static async queryCrowdloanRewardsTotalRewardsTest() {
        const totalRewards = await api.query.crowdloanRewards.totalRewards();
        (0, chai_1.expect)(totalRewards.toNumber()).to.be.a('number');
    }
}
exports.QueryCrowdloanRewardsTests = QueryCrowdloanRewardsTests;
