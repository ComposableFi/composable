import {
  fetchRewardPools,
  formatDurationOption,
  transformRewardPool,
} from "@/defi/polkadot/pallets/StakingRewards";
import BigNumber from "bignumber.js";
import { ApiPromise } from "@polkadot/api";

const rewardPool = {
  owner: "someone",
  assetId: 1,
  rewards: {},
  totalShares: 1_0000_0000_000,
  claimedShares: 100_000_000_000_000,
  endBlock: 0,
  lock: {
    durationPresets: {
      100: "100000000000",
      200: "100000000000",
    },
  },
  shareAssetId: 2001,
  financialNftAssetId: 3001,
};

describe("Staking reward integration", () => {
  it("formatDurationOption()", () => {
    const output = formatDurationOption("604800", new BigNumber(1));

    expect(output).toBe("1 week (1.00%)");
  });
  it("transformRewardPool()", () => {
    const output = transformRewardPool(rewardPool);

    expect(output.totalShares).toEqual(new BigNumber(1_0000_0000_000));
    expect(output.endBlock).toEqual(new BigNumber(0));
  });
  it("fetchRewardPools()", async () => {
    let mockedApi = {
      createType: jest.fn((a, b) => b),
      query: {
        stakingRewards: {
          rewardPools: jest.fn(() => ({
            toJSON: () => rewardPool,
            ...rewardPool,
          })),
        },
      },
      toJSON: jest.fn(() => ({})),
    } as unknown as ApiPromise;

    await fetchRewardPools(mockedApi, 1);
    expect(mockedApi.createType).toHaveBeenCalledTimes(1);
    expect(mockedApi.query.stakingRewards.rewardPools).toHaveBeenCalledTimes(1);
    expect(mockedApi.query.stakingRewards.rewardPools).toHaveBeenCalledWith(1);
  });
});
