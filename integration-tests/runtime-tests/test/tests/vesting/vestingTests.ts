import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { sendAndWaitForSuccess, waitForBlocks } from "@composable/utils/polkadotjs";
import { expect } from "chai";



// Vesting pallet integration test

// In these tests we are testing the following extrinsics:
//  - claim
//  - claimFor
//  - updateVestingSchedules
//  - vestedTransfer

describe("Vesting Pallet Tests", function () {
  let api: ApiPromise;
  let eth: number, usdt: number;
  let walletEveId: KeyringPair, walletFerdieId: KeyringPair, sudoKey: KeyringPair;
  let vestingScheduleIdSet, beneficiaryWallet;
  this.timeout(2 * 60 * 1000);

  before('Initialize variables', async function () {
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice, devWalletEve, devWalletFerdie } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    walletEveId = devWalletEve.derive("/test/constantProductDex/walletId1");
    walletFerdieId = devWalletFerdie.derive("/test/constantProductDex/walletId2");
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

//   it.only("debug", async function () {
//     const currentBlock = await waitForBlocks(api)
//     console.log("Number(waitForblocks) returns: ", Number(currentBlock));
//     console.log("waitForblocks.toString() returns: ", currentBlock.toString());
//     const from = api.createType("MultiAddress", {
//         id: walletEveId.address
//     });
//     console.log(" from: ",  from);
//   });
    
  // vestedTransfer extrinsic
  it("should perform a vested transfer - block number based", async function() {
    console.log("vested transfer function");
    const from = api.createType("MultiAddress", {
        id: walletEveId.address
    });
    const beneficiary = api.createType("MultiAddress", {
        id: walletFerdieId.address
    });
    const asset = api.createType("u128", usdt);
    const currentBlock = await waitForBlocks(api)
    const scheduleInfo = api.createType("ComposableTraitsVestingVestingScheduleInfo", {
        window: api.createType("ComposableTraitsVestingVestingWindow", {
            blockNumberBased: {
                start: api.createType("u32", (Number(currentBlock) + 5)),
                period: api.createType("u32", 2)
            }
        }),
        periodCount: 10,
        perPeriod: api.createType("u128", 10000000000000)
    });

    const { 
      data: [sender, recipient, assetId, vestingScheduleId, ] 
    } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.vesting.VestingScheduleAdded.is,
        api.tx.sudo.sudo(api.tx.vesting.vestedTransfer(from, beneficiary, asset, scheduleInfo))
    );

    vestingScheduleIdSet = Number(vestingScheduleId);
    beneficiaryWallet = recipient.toString();
    
 
    console.log("Number(assetId): ", Number(assetId));


    // console.log("lockId: ", lockId);
    // console.log("currencyId: ", currencyId);
    // console.log("who: ", who);
    // console.log("amount: ", amount);
    // console.log("usdt: ", usdt);
    // console.log("beneficiary: ", beneficiary);
    // console.log("suma : ", (20*10000000000000));


    // expect(sender.toString()).to.be.equal((walletEveId.address).toString());
    // expect(recipient.toString()).to.be.equal((walletFerdieId.address).toString());
    // expect(assetId.toString()).to.be.equal(asset.toString());

    // console.log("scheduleInfoInTx: ", scheduleInfoInTx);

    });
  // claim extrinsic
  it('should claim USDT', async function () {
    await waitForBlocks(api, 5);
    console.log("Claim function");
    const asset = api.createType("u128", usdt);
    const vestingScheduleId = api.createType("ComposableTraitsVestingVestingScheduleIdSet", {
        One: api.createType("u128", vestingScheduleIdSet)
    }); // ask

    await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.vesting.Claimed.is,
        api.tx.vesting.claim(asset, vestingScheduleId)
    );
    
    
  });

  // claimFor extrinsic
  it("should use claimFor extrinsic - USDT", async function () {
    console.log("Claim For function");
    this.timeout(2 * 60 * 1000);
    const target = api.createType("MultiAddress", {
        id: beneficiaryWallet
    });
    const asset = api.createType("u128", usdt);
    const vestingScheduleIds = api.createType("ComposableTraitsVestingVestingScheduleIdSet", "All");

    await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.vesting.Claimed.is,
        api.tx.vesting.claimFor(target, asset, vestingScheduleIds)
    );

  });

  // updateVestingSchedules extrinsic
  it("should update vesting schedule - Block number based", async function() {
    console.log("update function");
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
        },
      ]);

    await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.vesting.VestingSchedulesUpdated.is,
        api.tx.sudo.sudo(api.tx.vesting.updateVestingSchedules(who, asset, vestingSchedules))
    );

  });

});