import { ApiPromise } from "@polkadot/api";
/**
 * Contains all Query tests for the pallet:
 * crowdloanRewards
 */
export declare class QueryCrowdloanRewardsTests {
    /**
     * Checks for a successful return of
     * query.crowdloanRewards.claimedRewards()
     */
    static queryCrowdloanRewardsClaimedRewardsTest(api: ApiPromise): Promise<import("@polkadot/types-codec").u128>;
    /**
     * Checks for a successful return of
     * query.crowdloanRewards.totalContributors()
     */
    static queryCrowdloanRewardsTotalContributorsTest(api: ApiPromise): Promise<import("@polkadot/types-codec").u32>;
    /**
     * Checks for a successful return of
     * query.crowdloanRewards.totalRewards()
     */
    static queryCrowdloanRewardsTotalRewardsTest(api: ApiPromise): Promise<import("@polkadot/types-codec").u128>;
}
