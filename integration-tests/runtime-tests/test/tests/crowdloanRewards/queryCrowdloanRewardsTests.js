"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.QueryCrowdloanRewardsTests = void 0;
const chai_1 = require("chai");
const connectionHelper_1 = require("@composable/utils/connectionHelper");
const test_configuration_json_1 = __importDefault(require("@composabletests/tests/crowdloanRewards/test_configuration.json"));
describe("query.crowdloanRewards.account Tests", function () {
    if (!test_configuration_json_1.default.enabledTests.query.enabled)
        return;
    // Set timeout to 1 minute.
    this.timeout(60 * 1000);
    let api;
    before("Setting up tests", async function () {
        const { newClient } = await (0, connectionHelper_1.getNewConnection)();
        api = newClient;
    });
    after("Closing the connection", async function () {
        await api.disconnect();
    });
    it("query.crowdloanRewards.claimedRewards Tests", async function () {
        if (!test_configuration_json_1.default.enabledTests.query.claimedRewards_success)
            this.skip();
        const claimedRewards = await QueryCrowdloanRewardsTests.queryCrowdloanRewardsClaimedRewardsTest(api);
        (0, chai_1.expect)(claimedRewards.toNumber()).to.be.a("number");
    });
    it("query.crowdloanRewards.totalContributors Tests", async function () {
        if (!test_configuration_json_1.default.enabledTests.query.totalContributors_success)
            this.skip();
        const totalContributors = await QueryCrowdloanRewardsTests.queryCrowdloanRewardsTotalContributorsTest(api);
        (0, chai_1.expect)(totalContributors.toNumber()).to.be.a("number");
    });
    it("query.crowdloanRewards.totalRewards Tests", async function () {
        if (!test_configuration_json_1.default.enabledTests.query.totalRewards_success)
            this.skip();
        const totalRewards = await QueryCrowdloanRewardsTests.queryCrowdloanRewardsTotalRewardsTest(api);
        (0, chai_1.expect)(totalRewards.toNumber()).to.be.a("number");
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
    static async queryCrowdloanRewardsClaimedRewardsTest(api) {
        return await api.query.crowdloanRewards.claimedRewards();
    }
    /**
     * Checks for a successful return of
     * query.crowdloanRewards.totalContributors()
     */
    static async queryCrowdloanRewardsTotalContributorsTest(api) {
        return await api.query.crowdloanRewards.totalContributors();
    }
    /**
     * Checks for a successful return of
     * query.crowdloanRewards.totalRewards()
     */
    static async queryCrowdloanRewardsTotalRewardsTest(api) {
        return await api.query.crowdloanRewards.totalRewards();
    }
}
exports.QueryCrowdloanRewardsTests = QueryCrowdloanRewardsTests;
//# sourceMappingURL=queryCrowdloanRewardsTests.js.map