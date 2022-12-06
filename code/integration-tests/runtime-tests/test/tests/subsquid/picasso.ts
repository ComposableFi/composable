import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import {
  ActiveUsers,
  GET_ACTIVE_USERS,
  OVERVIEW_STATS,
  OverviewStats,
  GET_TOTAL_VALUE_LOCKED,
  TotalValueLocked
} from "@composable/utils/subsquid/apollo/queries";

import { client } from "@composable/utils/subsquid/apollo/apolloGraphql";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { sendAndWaitForSuccess, waitForBlocks } from "@composable/utils/polkadotjs";
import { getDevWallets } from "@composable/utils/walletHelper";
import { KeyringPair } from "@polkadot/keyring/types";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";

describe("Picasso graphql queries", function () {
  let api: ApiPromise;
  let sudoKey: KeyringPair, bobWallet: KeyringPair;

  before("Setting up the tests", async function () {
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    const { devWalletAlice, devWalletBob } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    bobWallet = devWalletBob.derive("/tests/assets/transferTestSenderWallet");
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  it("Correctly populates overview data", async function () {
    this.timeout(2 * 60 * 1000);
    this.retries(0);
    const paraAsset = api.createType("u128", 4);
    const paraDest = sudoKey.derive("/tests/assets/transferTestReceiverWallet1").publicKey;
    const paraAmount = api.createType("Balance", 100000000000);
    const paraKeepAlive = api.createType("bool", true);

    await mintAssetsToWallet(api, bobWallet, sudoKey, [1, 4], BigInt(1_000_000_000_000_000));

    await sendAndWaitForSuccess(
      api,
      bobWallet,
      api.events.balances.Deposit.is,
      api.tx.assets.transfer(paraAsset, paraDest, paraAmount, paraKeepAlive)
    );

    const { data } = await client.query<OverviewStats>({ query: OVERVIEW_STATS });

    expect(data.overviewStats).to.have.keys([
      "__typename",
      "accountHoldersCount",
      "activeUsersCount",
      "totalValueLocked",
      "transactionsCount"
    ]);

    expect(data.overviewStats.accountHoldersCount).to.equal(3);
    expect(data.overviewStats.activeUsersCount).to.equal(3);
    expect(data.overviewStats.totalValueLocked).to.equal("0");
    expect(data.overviewStats.transactionsCount).not.to.equal(0);
  });

  it("Gets active user chart for last day", async function () {
    const { data: dayData } = await client.query<ActiveUsers>({ query: GET_ACTIVE_USERS, variables: { range: "day" } });
    const { activeUsers } = dayData;
    // Should have one entry per hour
    expect(activeUsers.length).to.equal(24);
    // Last hour should have some activity
    expect(activeUsers[activeUsers.length - 1].count).not.to.equal(0);
  });

  it("Gets active user chart for last week", async function () {
    const { data: weekData } = await client.query<ActiveUsers>({
      query: GET_ACTIVE_USERS,
      variables: { range: "week" }
    });
    const { activeUsers } = weekData;
    // Should have one entry per day
    expect(activeUsers.length).to.equal(7);
    // Last day should have some activity
    expect(activeUsers[activeUsers.length - 1].count).not.to.equal(0);
  });

  it("Gets active user chart for last month", async function () {
    const { data: monthData } = await client.query<ActiveUsers>({
      query: GET_ACTIVE_USERS,
      variables: { range: "month" }
    });
    const { activeUsers } = monthData;
    // Should have one entry per day
    expect(activeUsers.length).to.equal(30);
    // Last day should have some activity
    expect(activeUsers[activeUsers.length - 1].count).not.to.equal(0);
  });

  it("Gets active user chart for last year", async function () {
    const { data: yearData } = await client.query<ActiveUsers>({
      query: GET_ACTIVE_USERS,
      variables: { range: "year" }
    });
    const { activeUsers } = yearData;
    // Should have one entry per month
    expect(activeUsers.length).to.equal(12);
    // Last month should have some activity
    expect(activeUsers[activeUsers.length - 1].count).not.to.equal(0);
  });

  it("Set up locked value tests", async function () {
    this.timeout(2 * 60 * 1000);
    await mintAssetsToWallet(api, sudoKey, sudoKey, [1, 4], BigInt(1_000_000_000_000_000));

    const from = api.createType("MultiAddress", {
      id: sudoKey.address
    });
    const beneficiary = api.createType("MultiAddress", {
      id: bobWallet.address
    });
    const asset = api.createType("u128", 1);
    const currentBlock = await waitForBlocks(api);
    const startBlock = Number(currentBlock) + 2;
    const windowPeriod = 1;
    const vestingPeriodCount = 10;
    const perPeriodAmount = 1_000_000_000_000;
    const scheduleInfo = api.createType("ComposableTraitsVestingVestingScheduleInfo", {
      window: api.createType("ComposableTraitsVestingVestingWindow", {
        blockNumberBased: {
          start: api.createType("u32", startBlock),
          period: api.createType("u32", windowPeriod)
        }
      }),
      periodCount: vestingPeriodCount,
      perPeriod: api.createType("u128", perPeriodAmount)
    });

    await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.vesting.VestingScheduleAdded.is,
      api.tx.sudo.sudo(api.tx.vesting.vestedTransfer(from, beneficiary, asset, scheduleInfo))
    );

    // This gives enough time for Subsquid to process the VestingScheduleAdded event for the next tests
    await waitForBlocks(api, 2);
  });

  it("Gets total value locked for last day", async function () {
    const { data: dayDataAll } = await client.query<TotalValueLocked>({
      query: GET_TOTAL_VALUE_LOCKED,
      variables: { range: "day" }
    });
    const { totalValueLocked: totalValueLockedAll } = dayDataAll;

    // Should have one entry per hour
    expect(totalValueLockedAll.length).to.equal(24);
    expect(totalValueLockedAll.every(({ source }) => source === "All")).to.be.true;

    const { data: dayDataVesting } = await client.query<TotalValueLocked>({
      query: GET_TOTAL_VALUE_LOCKED,
      variables: { range: "day", source: "VestingSchedules" }
    });
    const { totalValueLocked: totalValueLockedVesting } = dayDataVesting;

    expect(totalValueLockedVesting.length).to.equal(24);
    expect(totalValueLockedVesting.every(({ source }) => source === "VestingSchedules")).to.be.true;

    // Note: locked value will be 0 for now, as we don't have the Oracle to get asset prices
  });

  it("Gets total value locked for last week", async function () {
    // Note: locked value will be 0 for now, as we don't have the Oracle to get asset prices
    const { data: weekDataAll } = await client.query<TotalValueLocked>({
      query: GET_TOTAL_VALUE_LOCKED,
      variables: { range: "week" }
    });
    const { totalValueLocked: totalValueLockedAll } = weekDataAll;

    // Should have one entry per day
    expect(totalValueLockedAll.length).to.equal(7);
    expect(totalValueLockedAll.every(({ source }) => source === "All")).to.be.true;

    const { data: weekDataVesting } = await client.query<TotalValueLocked>({
      query: GET_TOTAL_VALUE_LOCKED,
      variables: { range: "week", source: "VestingSchedules" }
    });
    const { totalValueLocked: totalValueLockedVesting } = weekDataVesting;

    expect(totalValueLockedVesting.length).to.equal(7);
    expect(totalValueLockedVesting.every(({ source }) => source === "VestingSchedules")).to.be.true;
  });

  it("Gets total value locked for last month", async function () {
    // Note: locked value will be 0 for now, as we don't have the Oracle to get asset prices
    const { data: monthDataAll } = await client.query<TotalValueLocked>({
      query: GET_TOTAL_VALUE_LOCKED,
      variables: { range: "month" }
    });
    const { totalValueLocked: totalValueLockedAll } = monthDataAll;

    // Should have one entry per day
    expect(totalValueLockedAll.length).to.equal(30);
    expect(totalValueLockedAll.every(({ source }) => source === "All")).to.be.true;

    const { data: monthDataVesting } = await client.query<TotalValueLocked>({
      query: GET_TOTAL_VALUE_LOCKED,
      variables: { range: "month", source: "VestingSchedules" }
    });
    const { totalValueLocked: totalValueLockedVesting } = monthDataVesting;

    expect(totalValueLockedVesting.length).to.equal(30);
    expect(totalValueLockedVesting.every(({ source }) => source === "VestingSchedules")).to.be.true;
  });

  it("Gets total value locked for last year", async function () {
    // Note: locked value will be 0 for now, as we don't have the Oracle to get asset prices
    const { data: yearDataAll } = await client.query<TotalValueLocked>({
      query: GET_TOTAL_VALUE_LOCKED,
      variables: { range: "year" }
    });
    const { totalValueLocked: totalValueLockedAll } = yearDataAll;

    // Should have one entry per month
    expect(totalValueLockedAll.length).to.equal(12);
    expect(totalValueLockedAll.every(({ source }) => source === "All")).to.be.true;

    const { data: yearDataVesting } = await client.query<TotalValueLocked>({
      query: GET_TOTAL_VALUE_LOCKED,
      variables: { range: "year", source: "VestingSchedules" }
    });
    const { totalValueLocked: totalValueLockedVesting } = yearDataVesting;

    expect(totalValueLockedVesting.length).to.equal(12);
    expect(totalValueLockedVesting.every(({ source }) => source === "VestingSchedules")).to.be.true;
  });
});
