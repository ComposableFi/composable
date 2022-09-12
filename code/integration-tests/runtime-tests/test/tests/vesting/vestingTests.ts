import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { sendAndWaitForSuccess, waitForBlocks } from "@composable/utils/polkadotjs";
import { expect } from "chai";

// Vesting pallet integration test

// In these tests we are testing the following extrinsics:
//  - vestedTransfer
//  - claim
//  - claimFor
//  - updateVestingSchedules

describe("Vesting Pallet Tests", function () {
  let api: ApiPromise;
  let eth: number, usdt: number;
  let wallet1: KeyringPair, wallet2: KeyringPair, sudoKey: KeyringPair;
  let vestingScheduleIdSet: any;
  this.timeout(2 * 60 * 1000);

  before("Initialize variables", async function () {
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice, devWalletEve, devWalletFerdie } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    wallet1 = devWalletEve.derive("/test/vesting/1");
    wallet2 = devWalletFerdie.derive("/test/vesting/2");
    eth = 5;
    usdt = 130;
  });

  before("Minting assets", async function () {
    await mintAssetsToWallet(api, wallet1, sudoKey, [1, eth, usdt]);
    await mintAssetsToWallet(api, wallet2, sudoKey, [1, eth, usdt]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  it("should perform a vested transfer - block number based", async function () {
    const from = api.createType("MultiAddress", {
      id: wallet1.address
    });
    const beneficiary = api.createType("MultiAddress", {
      id: wallet2.address
    });
    const asset = api.createType("u128", usdt);
    const currentBlock = await waitForBlocks(api);
    const startBlock = Number(currentBlock) + 2;
    const windowPeriod = 2;
    const vestingPeriodCount = 10;
    const perPeriodAmount = 500000000000000;
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

    const rawInitialBalance1 = await api.query.tokens.accounts(wallet1.address, usdt);
    const initialBalance1 = rawInitialBalance1.toJSON();

    const rawInitialBalance2 = await api.query.tokens.accounts(wallet2.address, usdt);
    const initialBalance2 = rawInitialBalance2.toJSON();

    const {
      data: [, currencyId, walletId, amount]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.tokens.LockSet.is,
      api.tx.sudo.sudo(api.tx.vesting.vestedTransfer(from, beneficiary, asset, scheduleInfo))
    );

    const rawFinalBalance1 = await api.query.tokens.accounts(wallet1.address, usdt);
    const finalBalance1 = rawFinalBalance1.toJSON();

    const rawFinalBalance2 = await api.query.tokens.accounts(wallet2.address, usdt);
    const finalBalance2 = rawFinalBalance2.toJSON();

    const rawAnswer = await api.query.vesting.vestingSchedules(wallet2.address, usdt);
    const answer = rawAnswer.toJSON();
    // @ts-ignore
    vestingScheduleIdSet = answer["1"]["vestingScheduleId"];

    // @ts-ignore
    expect(answer["1"]["window"]["blockNumberBased"]["start"]).to.be.eq(startBlock);
    // @ts-ignore
    expect(answer["1"]["window"]["blockNumberBased"]["period"]).to.be.eq(windowPeriod);
    // @ts-ignore
    expect(answer["1"]["periodCount"]).to.be.eq(vestingPeriodCount);
    // @ts-ignore
    expect(answer["1"]["perPeriod"]).to.be.eq(perPeriodAmount);
    expect(api.createType("AccountId32", wallet2.publicKey).toString()).to.be.eq(walletId.toString());
    expect(Number(initialBalance1["free"])).to.be.gt(Number(finalBalance1["free"]));
    expect(Number(initialBalance2["free"])).to.be.lt(Number(finalBalance2["free"]));
    expect(Number(currencyId)).to.be.eq(usdt);
    expect(Number(amount)).to.be.eq(vestingPeriodCount * perPeriodAmount);
  });

  it("beneficiary should be able to claim available funds", async function () {
    await waitForBlocks(api, 4);
    const asset = api.createType("u128", usdt);
    const vestingScheduleId = api.createType("ComposableTraitsVestingVestingScheduleIdSet", "All");

    const rawInitialBalance2 = await api.query.tokens.accounts(wallet2.address, usdt);
    const initialBalance2 = rawInitialBalance2.toJSON();

    const {
      data: [, assetId, , lockedAmount, claimedAmount]
    } = await sendAndWaitForSuccess(
      api,
      wallet2,
      api.events.vesting.Claimed.is,
      api.tx.vesting.claim(asset, vestingScheduleId)
    );

    const rawFinalBalance2 = await api.query.tokens.accounts(wallet2.address, usdt);
    const finalBalance2 = rawFinalBalance2.toJSON();

    expect(Number(initialBalance2["frozen"])).to.be.gt(Number(finalBalance2["frozen"]));
    expect(Number(assetId)).to.be.eq(usdt);
    expect(Number(lockedAmount)).to.be.gt(0);
    expect(Number(claimedAmount)).to.be.gt(0);
  });

  it("any user can claim available funds for beneficiary", async function () {
    const target = api.createType("MultiAddress", {
      id: wallet2.address
    });
    const asset = api.createType("u128", usdt);
    const vestingScheduleIds = api.createType("ComposableTraitsVestingVestingScheduleIdSet", "All");

    const rawInitialBalance2 = await api.query.tokens.accounts(wallet2.address, usdt);
    const initialBalance2 = rawInitialBalance2.toJSON();

    const {
      data: [, assetId, , lockedAmount, claimedAmount]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.vesting.Claimed.is,
      api.tx.vesting.claimFor(target, asset, vestingScheduleIds)
    );

    const rawFinalBalance2 = await api.query.tokens.accounts(wallet2.address, usdt);
    const finalBalance2 = rawFinalBalance2.toJSON();

    await waitForBlocks(api, 4);
    expect(Number(initialBalance2["frozen"])).to.be.gt(Number(finalBalance2["frozen"]));
    expect(Number(assetId)).to.be.eq(usdt);
    expect(Number(lockedAmount)).to.be.gt(0);
    expect(Number(claimedAmount)).to.be.gt(0);
  });

  it("should update vesting schedule - Block number based", async function () {
    const who = api.createType("MultiAddress", {
      id: wallet1.address
    });
    const asset = api.createType("u128", usdt);
    const startBlock = 55;
    const windowPeriod = 1;
    const vestingPeriodCount = 40;
    const perPeriodAmount = 1000000000000;
    const vestingSchedules = api.createType("Vec<ComposableTraitsVestingVestingSchedule>", [
      {
        vestingScheduleId: api.createType("u128", vestingScheduleIdSet),
        window: api.createType("ComposableTraitsVestingVestingWindow", {
          blockNumberBased: {
            start: api.createType("u32", startBlock),
            period: api.createType("u32", windowPeriod)
          }
        }),
        periodCount: vestingPeriodCount,
        perPeriod: api.createType("Compact<u128>", perPeriodAmount),
        alreadyClaimed: api.createType("u128", 0)
      }
    ]);

    const {
      data: [, currencyId, , amount]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.tokens.LockSet.is,
      api.tx.sudo.sudo(api.tx.vesting.updateVestingSchedules(who, asset, vestingSchedules))
    );

    const rawFinalAnswer = await api.query.vesting.vestingSchedules(wallet1.address, usdt);
    const finalAnswer = rawFinalAnswer.toJSON();

    // @ts-ignore
    expect(finalAnswer["2"]["window"]["blockNumberBased"]["start"]).to.be.eq(startBlock);
    // @ts-ignore
    expect(finalAnswer["2"]["window"]["blockNumberBased"]["period"]).to.be.eq(windowPeriod);
    // @ts-ignore
    expect(finalAnswer["2"]["periodCount"]).to.be.eq(vestingPeriodCount);
    // @ts-ignore
    expect(finalAnswer["2"]["perPeriod"]).to.be.eq(perPeriodAmount);
    expect(Number(currencyId)).to.be.eq(usdt);
    expect(Number(amount)).to.be.eq(40 * 1000000000000);
  });

  it("should perform a vested transfer - moment based", async function () {
    const from = api.createType("MultiAddress", {
      id: wallet1.address
    });
    const beneficiary = api.createType("MultiAddress", {
      id: wallet2.address
    });
    const asset = api.createType("u128", usdt);
    const startBlock = 18;
    const windowPeriod = 2;
    const vestingPeriodCount = 10;
    const perPeriodAmount = 10000000000000;
    const scheduleInfo = api.createType("ComposableTraitsVestingVestingScheduleInfo", {
      window: api.createType("ComposableTraitsVestingVestingWindow", {
        MomentBased: {
          start: api.createType("u64", startBlock),
          period: api.createType("u64", windowPeriod)
        }
      }),
      periodCount: vestingPeriodCount,
      perPeriod: api.createType("u128", perPeriodAmount)
    });

    const rawInitialBalance1 = await api.query.tokens.accounts(wallet1.address, usdt);
    const initialBalance1 = rawInitialBalance1.toJSON();

    const rawInitialBalance2 = await api.query.tokens.accounts(wallet2.address, usdt);
    const initialBalance2 = rawInitialBalance2.toJSON();

    const {
      data: [, currencyId, walletId]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.tokens.LockSet.is,
      api.tx.sudo.sudo(api.tx.vesting.vestedTransfer(from, beneficiary, asset, scheduleInfo))
    );

    const rawFinalBalance1 = await api.query.tokens.accounts(wallet1.address, usdt);
    const finalBalance1 = rawFinalBalance1.toJSON();

    const rawFinalBalance2 = await api.query.tokens.accounts(wallet2.address, usdt);
    const finalBalance2 = rawFinalBalance2.toJSON();

    const rawAnswer = await api.query.vesting.vestingSchedules(wallet2.address, usdt);
    const answer = rawAnswer.toJSON();

    // @ts-ignore
    expect(answer["3"]["window"]["momentBased"]["start"]).to.be.eq(startBlock);
    // @ts-ignore
    expect(answer["3"]["window"]["momentBased"]["period"]).to.be.eq(windowPeriod);
    // @ts-ignore
    expect(answer["3"]["periodCount"]).to.be.eq(vestingPeriodCount);
    // @ts-ignore
    expect(answer["3"]["perPeriod"]).to.be.eq(perPeriodAmount);
    expect(api.createType("AccountId32", wallet2.publicKey).toString()).to.be.eq(walletId.toString());
    expect(Number(initialBalance1["free"])).to.be.gt(Number(finalBalance1["free"]));
    expect(Number(initialBalance2["free"])).to.be.lt(Number(finalBalance2["free"]));
    expect(Number(currencyId)).to.be.eq(usdt);
  });
});
