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
  let walletEveId: KeyringPair, walletFerdieId: KeyringPair, sudoKey: KeyringPair;
  let vestingScheduleIdSet, beneficiaryWallet;
  this.timeout(2 * 60 * 1000);

  before("Initialize variables", async function () {
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice, devWalletEve, devWalletFerdie } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    walletEveId = devWalletEve;
    walletFerdieId = devWalletFerdie;
    eth = 5;
    usdt = 6;
  });

  before("Minting assets", async function () {
    await mintAssetsToWallet(api, walletEveId, sudoKey, [1, eth, usdt]);
    await mintAssetsToWallet(api, walletFerdieId, sudoKey, [1, eth, usdt]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  it("should perform a vested transfer - block number based", async function () {
    const from = api.createType("MultiAddress", {
      id: walletEveId.address
    });
    const beneficiary = api.createType("MultiAddress", {
      id: walletFerdieId.address
    });
    const asset = api.createType("u128", usdt);
    const currentBlock = await waitForBlocks(api);
    const scheduleInfo = api.createType("ComposableTraitsVestingVestingScheduleInfo", {
      window: api.createType("ComposableTraitsVestingVestingWindow", {
        blockNumberBased: {
          start: api.createType("u32", Number(currentBlock) + 2),
          period: api.createType("u32", 2)
        }
      }),
      periodCount: 10,
      perPeriod: api.createType("u128", 10000000000000)
    });

    const {
      data: [, currencyId, , amount]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.tokens.LockSet.is,
      api.tx.sudo.sudo(api.tx.vesting.vestedTransfer(from, beneficiary, asset, scheduleInfo))
    );

    const rawAnswer = await api.query.vesting.vestingSchedules(walletFerdieId.address, usdt);
    const answer = rawAnswer.toJSON();
    vestingScheduleIdSet = answer["1"]["vestingScheduleId"];

    expect(Number(currencyId)).to.be.eq(usdt);
    expect(Number(amount)).to.be.eq(10 * 10000000000000);
  });

  it("beneficiary should be able to claim available funds", async function () {
    await waitForBlocks(api, 4);
    const asset = api.createType("u128", usdt);
    const vestingScheduleId = api.createType("ComposableTraitsVestingVestingScheduleIdSet", "All");

    const {
      data: [, assetId, , lockedAmount, claimedAmount]
    } = await sendAndWaitForSuccess(
      api,
      walletFerdieId,
      api.events.vesting.Claimed.is,
      api.tx.vesting.claim(asset, vestingScheduleId)
    );

    expect(Number(assetId)).to.be.eq(usdt);
    expect(Number(lockedAmount)).to.be.gt(0);
    expect(Number(claimedAmount)).to.be.gt(0);
  });

  it("any user can claim available funds for beneficiary", async function () {
    const target = api.createType("MultiAddress", {
      id: walletFerdieId.address
    });
    const asset = api.createType("u128", usdt);
    const vestingScheduleIds = api.createType("ComposableTraitsVestingVestingScheduleIdSet", "All");

    const {
      data: [, assetId, , lockedAmount, claimedAmount]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.vesting.Claimed.is,
      api.tx.vesting.claimFor(target, asset, vestingScheduleIds)
    );

    await waitForBlocks(api, 4);
    expect(Number(assetId)).to.be.eq(usdt);
    expect(Number(lockedAmount)).to.be.gt(0);
    expect(Number(claimedAmount)).to.be.gt(0);
  });

  it("should update vesting schedule - Block number based", async function () {
    const who = api.createType("MultiAddress", {
      id: walletEveId.address
    });
    const asset = api.createType("u128", usdt);
    const vestingSchedules = api.createType("Vec<ComposableTraitsVestingVestingSchedule>", [
      {
        vestingScheduleId: api.createType("u128", vestingScheduleIdSet),
        window: api.createType("ComposableTraitsVestingVestingWindow", {
          blockNumberBased: {
            start: api.createType("u32", 55),
            period: api.createType("u32", 2)
          }
        }),
        periodCount: 40,
        perPeriod: api.createType("Compact<u128>", 1000000000000),
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

    expect(Number(currencyId)).to.be.eq(usdt);
    expect(Number(amount)).to.be.eq(40 * 1000000000000);
  });

  it("should perform a vested transfer - moment based", async function () {
    const from = api.createType("MultiAddress", {
      id: walletEveId.address
    });
    const beneficiary = api.createType("MultiAddress", {
      id: walletFerdieId.address
    });
    const asset = api.createType("u128", usdt);
    const scheduleInfo = api.createType("ComposableTraitsVestingVestingScheduleInfo", {
      window: api.createType("ComposableTraitsVestingVestingWindow", {
        MomentBased: {
          start: api.createType("u64", 18),
          period: api.createType("u64", 2)
        }
      }),
      periodCount: 10,
      perPeriod: api.createType("u128", 10000000000000)
    });

    const {
      data: [, currencyId, , amount]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.tokens.LockSet.is,
      api.tx.sudo.sudo(api.tx.vesting.vestedTransfer(from, beneficiary, asset, scheduleInfo))
    );

    expect(Number(currencyId)).to.be.eq(usdt);
  });
});
