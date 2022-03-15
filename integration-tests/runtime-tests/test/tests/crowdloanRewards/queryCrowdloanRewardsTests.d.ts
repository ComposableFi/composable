/**
 * Contains all Query tests for the pallet:
 * crowdloanRewards
 *
 * ToDo: Add additional checks.
 */
export declare class QueryCrowdloanRewardsTests {
    /**
     * Checks for a successful return of
     * query.crowdloanRewards.claimedRewards()
     */
    static queryCrowdloanRewardsClaimedRewardsTest(): Promise<void>;
    /**
     * Checks for a successful return of
     * query.crowdloanRewards.totalContributors()
     */
    static queryCrowdloanRewardsTotalContributorsTest(): Promise<void>;
    /**
     * Checks for a successful return of
     * query.crowdloanRewards.totalRewards()
     */
    static queryCrowdloanRewardsTotalRewardsTest(): Promise<void>;
}
