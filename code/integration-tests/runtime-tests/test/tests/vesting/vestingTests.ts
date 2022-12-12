import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import BN from "bn.js";
import { sendAndWaitForSuccess, sendWithBatchAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { vestedScheduleClaimVerifier, vestedScheduleCreationVerifier } from "@composabletests/tests/vesting/testHelper";
import { expect } from "chai";
import { u128 } from "@polkadot/types-codec";

const PICA_ASSET_ID = new BN(1);
const kUSD_ASSET_ID = new BN(129);

const SCHEDULE_AMOUNT_PER_PERIOD = new BN(10).pow(new BN(12));

const SCHEDULE_BLOCK_PERIOD = new BN(2);
const SCHEDULE_BLOCK_PERIOD_COUNT = new BN(10);

const SCHEDULE_MOMENT_PERIOD = new BN(10000);
const SCHEDULE_MOMENT_PERIOD_COUNT = new BN(100);

describe.skip("[SHORT] Vesting Pallet Tests", function () {
  let api: ApiPromise, api2: ApiPromise, api3: ApiPromise;
  let wallet1: KeyringPair,
    wallet2: KeyringPair,
    wallet3: KeyringPair,
    wallet4: KeyringPair,
    wallet5: KeyringPair,
    sudoKey: KeyringPair;

  let vestingScheduleId1: BN, vestingScheduleId2: BN, vestingScheduleId3: BN;
  let vestingScheduleIdCollection: u128[];

  let vestingSchedule1StartBlock: BN, vestingSchedule3StartBlock: BN;
  let vestingSchedule1EndBlock: BN, vestingSchedule3EndBlock: BN;

  let vestingSchedule2StartTime: BN, vestingSchedule2EndTime: BN;

  let claimedAmountsSchedule1: BN, claimedAmountsSchedule2: BN, claimedAmountsSchedule3: BN;

  before("Initialize variables", async function () {
    this.timeout(5 * 60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice, devWalletEve } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    wallet1 = devWalletEve.derive("/test/vesting/1");
    wallet2 = devWalletEve.derive("/test/vesting/2");
    wallet3 = devWalletEve.derive("/test/vesting/3");
    wallet4 = devWalletEve.derive("/test/vesting/4");
    wallet5 = devWalletEve.derive("/test/vesting/5");

    await mintAssetsToWallet(api, wallet1, sudoKey, [1, 129], 999_999_999_999_999_999n);
    await mintAssetsToWallet(api, wallet2, sudoKey, [1]);
    await mintAssetsToWallet(api, wallet3, sudoKey, [1]);
    await mintAssetsToWallet(api, wallet4, sudoKey, [1]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
    await api2.disconnect();
    await api3.disconnect();
  });

  it("#1.1  I can, as sudo, make a vested transfer which is block number based.", async function () {
    this.timeout(2 * 60 * 1000);

    // Parameters
    const walletFundSender = wallet1;
    const walletBeneficiary = wallet2;
    const assetId = PICA_ASSET_ID;
    const currentBlockNum = await api.query.system.number();
    const startBlock = currentBlockNum.add(new BN(2));
    const schedulePeriodCount = SCHEDULE_BLOCK_PERIOD_COUNT;
    const schedulePerPeriod = SCHEDULE_AMOUNT_PER_PERIOD;
    const scheduleInfo = api.createType("ComposableTraitsVestingVestingScheduleInfo", {
      window: api.createType("ComposableTraitsVestingVestingWindow", {
        blockNumberBased: {
          start: api.createType("BlockNumber", startBlock),
          period: api.createType("BlockNumber", SCHEDULE_BLOCK_PERIOD)
        }
      }),
      periodCount: schedulePeriodCount,
      perPeriod: schedulePerPeriod // 1 PICA per period
    });

    vestingSchedule1StartBlock = startBlock;
    vestingSchedule1EndBlock = startBlock.add(
      scheduleInfo.window.asBlockNumberBased.period.mul(scheduleInfo.periodCount)
    );

    const verificationHandler = new vestedScheduleCreationVerifier(api);
    await verificationHandler.verificationSetup(assetId, walletFundSender.publicKey);

    const results = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.vesting.VestingScheduleAdded.is,
      api.tx.sudo.sudo(
        api.tx.vesting.vestedTransfer(walletFundSender.publicKey, walletBeneficiary.publicKey, assetId, scheduleInfo)
      )
    );
    await verificationHandler.verifyVestedScheduleCreation(
      walletFundSender.publicKey,
      walletBeneficiary.publicKey,
      assetId,
      schedulePerPeriod,
      schedulePeriodCount,
      scheduleInfo,
      results.data,
      "block"
    );
    vestingScheduleId1 = results.data[3];
  });

  it("#1.2  I can, as sudo, make a vested transfer which is timestamp based.", async function () {
    this.timeout(2 * 60 * 1000);

    // Parameters
    const walletFundSender = wallet1;
    const walletBeneficiary = wallet3;
    const assetId = PICA_ASSET_ID;
    const currentTimestamp = new BN(Date.now());
    const momentStart = currentTimestamp.add(new BN(100));
    const schedulePeriodCount = SCHEDULE_MOMENT_PERIOD_COUNT;
    const schedulePerPeriod = SCHEDULE_AMOUNT_PER_PERIOD;
    const scheduleInfo = api.createType("ComposableTraitsVestingVestingScheduleInfo", {
      window: api.createType("ComposableTraitsVestingVestingWindow", {
        MomentBased: {
          start: api.createType("Moment", momentStart),
          period: api.createType("Moment", SCHEDULE_MOMENT_PERIOD)
        }
      }),
      periodCount: schedulePeriodCount,
      perPeriod: schedulePerPeriod // 1 PICA per period
    });

    const verificationHandler = new vestedScheduleCreationVerifier(api);
    await verificationHandler.verificationSetup(assetId, walletFundSender.publicKey);

    vestingSchedule2StartTime = momentStart;
    vestingSchedule2EndTime = vestingSchedule2StartTime.add(
      scheduleInfo.window.asMomentBased.period.mul(scheduleInfo.periodCount)
    );
    const results = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.vesting.VestingScheduleAdded.is,
      api.tx.sudo.sudo(
        api.tx.vesting.vestedTransfer(walletFundSender.publicKey, walletBeneficiary.publicKey, assetId, scheduleInfo)
      )
    );
    await verificationHandler.verifyVestedScheduleCreation(
      walletFundSender.publicKey,
      walletBeneficiary.publicKey,
      assetId,
      schedulePerPeriod,
      schedulePeriodCount,
      scheduleInfo,
      results.data,
      "moment"
    );
    vestingScheduleId2 = results.data[3];
  });

  it("#1.3  I can, as sudo, make a vested transfer for any asset ID.", async function () {
    this.timeout(2 * 60 * 1000);

    // Parameters
    const walletFundSender = wallet1;
    const walletBeneficiary = wallet4;
    const assetId = kUSD_ASSET_ID;
    const currentBlockNum = await api.query.system.number();
    const startBlock = new BN(5).add(currentBlockNum);
    const period = SCHEDULE_BLOCK_PERIOD;
    const schedulePeriodCount = SCHEDULE_BLOCK_PERIOD_COUNT;
    const schedulePerPeriod = SCHEDULE_AMOUNT_PER_PERIOD;
    const scheduleInfo = api.createType("ComposableTraitsVestingVestingScheduleInfo", {
      window: api.createType("ComposableTraitsVestingVestingWindow", {
        BlockNumberBased: {
          start: api.createType("BlockNumber", startBlock),
          period: api.createType("BlockNumber", period)
        }
      }),
      periodCount: schedulePeriodCount,
      perPeriod: schedulePerPeriod // 1 PICA per period
    });

    vestingSchedule3StartBlock = startBlock;
    vestingSchedule3EndBlock = startBlock.add(
      scheduleInfo.window.asBlockNumberBased.period.mul(scheduleInfo.periodCount)
    );

    const verificationHandler = new vestedScheduleCreationVerifier(api);
    await verificationHandler.verificationSetup(assetId, walletFundSender.publicKey);

    const results = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.vesting.VestingScheduleAdded.is,
      api.tx.sudo.sudo(
        api.tx.vesting.vestedTransfer(walletFundSender.publicKey, walletBeneficiary.publicKey, assetId, scheduleInfo)
      )
    );
    await verificationHandler.verifyVestedScheduleCreation(
      walletFundSender.publicKey,
      walletBeneficiary.publicKey,
      assetId,
      schedulePerPeriod,
      schedulePeriodCount,
      scheduleInfo,
      results.data,
      "block"
    );
    vestingScheduleId3 = results.data[3];
  });

  it("#1.19 I can, as sudo, batch create multiple vested transfers.", async function () {
    this.timeout(2 * 60 * 1000);
    // Parameters
    const walletFundSender = wallet1;
    const walletBeneficiary = wallet5;

    const assetId = new BN(1);
    const currentBlockNum = await api.query.system.number();
    const startBlock = new BN(5).add(currentBlockNum);
    const schedulePeriodCount = SCHEDULE_BLOCK_PERIOD_COUNT;
    const scheduleInfo = api.createType("ComposableTraitsVestingVestingScheduleInfo", {
      window: api.createType("ComposableTraitsVestingVestingWindow", {
        blockNumberBased: {
          start: api.createType("BlockNumber", startBlock),
          period: api.createType("BlockNumber", SCHEDULE_BLOCK_PERIOD)
        }
      }),
      periodCount: schedulePeriodCount,
      perPeriod: SCHEDULE_AMOUNT_PER_PERIOD // 1 PICA per period
    });

    const txs = [
      api.tx.sudo.sudo(
        api.tx.vesting.vestedTransfer(walletFundSender.publicKey, walletBeneficiary.publicKey, assetId, scheduleInfo)
      ),
      api.tx.sudo.sudo(
        api.tx.vesting.vestedTransfer(walletFundSender.publicKey, walletBeneficiary.publicKey, assetId, scheduleInfo)
      ),
      api.tx.sudo.sudo(
        api.tx.vesting.vestedTransfer(walletFundSender.publicKey, walletBeneficiary.publicKey, assetId, scheduleInfo)
      )
    ];
    const results = await sendWithBatchAndWaitForSuccess(
      api,
      sudoKey,
      api.events.vesting.VestingScheduleAdded.is,
      txs,
      false
    );

    const vestingIds: u128[] = [];
    for (let i = 0; i < txs.length; i++) {
      // @ts-ignore
      vestingIds.push(api.createType("u128", results.valueOf(i).data[3].toNumber()));
    }
    vestingScheduleIdCollection = vestingIds;
  });

  it("#1.15 I can not make a vested transfer without sudo rights.", async function () {
    this.timeout(2 * 60 * 1000);
    // Parameters
    const walletFundSender = wallet1;
    const walletBeneficiary = wallet5;
    const assetId = new BN(129);
    const momentStart = 10;
    const momentPeriod = 10;
    const schedulePeriodCount = new BN(10);
    const schedulePerPeriod = new BN(10).pow(new BN(12));
    const scheduleInfo = api.createType("ComposableTraitsVestingVestingScheduleInfo", {
      window: api.createType("ComposableTraitsVestingVestingWindow", {
        BlockNumberBased: {
          start: api.createType("BlockNumber", momentStart),
          period: api.createType("BlockNumber", momentPeriod)
        }
      }),
      periodCount: schedulePeriodCount,
      perPeriod: schedulePerPeriod // 1 PICA per period
    });

    const verificationHandler = new vestedScheduleCreationVerifier(api);
    verificationHandler.verificationSetup(assetId, walletFundSender.publicKey);

    const res = await sendAndWaitForSuccess(
      api,
      walletFundSender,
      api.events.vesting.VestingScheduleAdded.is,
      api.tx.vesting.vestedTransfer(walletFundSender.publicKey, walletBeneficiary.publicKey, assetId, scheduleInfo)
    ).catch(exc => {
      return exc;
    });
    expect(res.toString()).to.contain("BadOrigin");
  });

  it("#1.5  The beneficiary of a block vested transfer (#1.1) can claim its transfer during the vesting period.", async function () {
    this.timeout(2 * 60 * 1000);

    const assetId = PICA_ASSET_ID;
    const vestingSchedule = api.createType("ComposableTraitsVestingVestingScheduleIdSet", {
      One: api.createType("u128", vestingScheduleId1)
    });
    const wallet = wallet2;
    const schedulePeriodCount = SCHEDULE_BLOCK_PERIOD_COUNT;
    const schedulePerPeriod = SCHEDULE_AMOUNT_PER_PERIOD;

    const verificationHandler = new vestedScheduleClaimVerifier(api, wallet.publicKey, assetId, vestingScheduleId1);
    await verificationHandler.verificationSetup();

    const results = await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.vesting.Claimed.is,
      api.tx.vesting.claim(assetId, vestingSchedule)
    );

    claimedAmountsSchedule1 = await verificationHandler.verifyVestedScheduleClaim(
      schedulePerPeriod,
      schedulePeriodCount,
      SCHEDULE_BLOCK_PERIOD,
      vestingSchedule1StartBlock,
      vestingSchedule1EndBlock,
      results,
      "block"
    );
  });

  it("#1.7  The beneficiary of a moment based vested transfer (#1.2) can claim its transfer during the vesting period.", async function () {
    this.timeout(2 * 60 * 1000);

    const assetId = PICA_ASSET_ID;
    const vestingSchedule = { One: vestingScheduleId2 };
    const wallet = wallet3;
    const schedulePeriodCount = SCHEDULE_MOMENT_PERIOD_COUNT;
    const schedulePerPeriod = SCHEDULE_AMOUNT_PER_PERIOD;

    const verificationHandler = new vestedScheduleClaimVerifier(api, wallet.publicKey, assetId, vestingScheduleId2);
    await verificationHandler.verificationSetup();

    const results = await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.vesting.Claimed.is,
      api.tx.vesting.claim(assetId, vestingSchedule)
    );

    claimedAmountsSchedule2 = await verificationHandler.verifyVestedScheduleClaim(
      schedulePerPeriod,
      schedulePeriodCount,
      SCHEDULE_MOMENT_PERIOD,
      vestingSchedule2StartTime,
      vestingSchedule2EndTime,
      results,
      "moment"
    );
  });

  it("#1.20  The beneficiary of a moment based vested transfer (#1.3) can claim its transfer during the vesting period.", async function () {
    this.timeout(2 * 60 * 1000);

    const assetId = kUSD_ASSET_ID;
    const vestingSchedule = { One: vestingScheduleId3 };
    const wallet = wallet4;
    const schedulePeriodCount = SCHEDULE_BLOCK_PERIOD_COUNT;
    const schedulePerPeriod = SCHEDULE_AMOUNT_PER_PERIOD;

    const verificationHandler = new vestedScheduleClaimVerifier(api, wallet.publicKey, assetId, vestingScheduleId3);
    await verificationHandler.verificationSetup();

    const results = await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.vesting.Claimed.is,
      api.tx.vesting.claim(assetId, vestingSchedule)
    );

    claimedAmountsSchedule3 = await verificationHandler.verifyVestedScheduleClaim(
      schedulePerPeriod,
      schedulePeriodCount,
      SCHEDULE_BLOCK_PERIOD,
      vestingSchedule3StartBlock,
      vestingSchedule3EndBlock,
      results,
      "block"
    );
  });

  it("#1.9  Any user can claim a transfer for someone else. #1.1", async function () {
    this.timeout(2 * 60 * 1000);

    const assetId = PICA_ASSET_ID;
    const vestingSchedule = { One: vestingScheduleId1 };
    const wallet = wallet1;
    const schedulePeriodCount = SCHEDULE_BLOCK_PERIOD_COUNT;
    const schedulePerPeriod = SCHEDULE_AMOUNT_PER_PERIOD;

    const verificationHandler = new vestedScheduleClaimVerifier(api, wallet2.publicKey, assetId, vestingScheduleId1);
    await verificationHandler.verificationSetup();

    const results = await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.vesting.Claimed.is,
      api.tx.vesting.claimFor(wallet2.publicKey, assetId, vestingSchedule)
    );

    claimedAmountsSchedule1 = claimedAmountsSchedule1.add(
      await verificationHandler.verifyVestedScheduleClaim(
        schedulePerPeriod,
        schedulePeriodCount,
        SCHEDULE_BLOCK_PERIOD,
        vestingSchedule1StartBlock,
        vestingSchedule1EndBlock,
        results,
        "block",
        claimedAmountsSchedule1
      )
    );
  });

  it("#1.4  Multiple vested transfer beneficiaries (#1.1, #1.2, #1.3) can claim simultaneously.", async function () {
    this.timeout(2 * 60 * 1000);
    const txWallet1 = wallet2;
    const txWallet2 = wallet3;
    const txWallet3 = wallet4;

    // We can't use the same api connection to make multiple transaction for some reason.
    const conn2 = await getNewConnection();
    const conn3 = await getNewConnection();
    api2 = conn2.newClient;
    api3 = conn3.newClient;

    const verificationHandler1 = new vestedScheduleClaimVerifier(
      api,
      txWallet1.publicKey,
      PICA_ASSET_ID,
      vestingScheduleId1
    );
    const verificationHandler2 = new vestedScheduleClaimVerifier(
      api2,
      txWallet2.publicKey,
      PICA_ASSET_ID,
      vestingScheduleId2
    );
    const verificationHandler3 = new vestedScheduleClaimVerifier(
      api3,
      txWallet3.publicKey,
      kUSD_ASSET_ID,
      vestingScheduleId3
    );
    await Promise.all([
      verificationHandler1.verificationSetup(),
      verificationHandler2.verificationSetup(),
      verificationHandler3.verificationSetup()
    ]);

    await Promise.all([
      sendAndWaitForSuccess(api, txWallet1, api.events.vesting.Claimed.is, api.tx.vesting.claim("1", "All")),
      sendAndWaitForSuccess(api2, txWallet2, api.events.vesting.Claimed.is, api.tx.vesting.claim("1", "All")),
      sendAndWaitForSuccess(api3, txWallet3, api.events.vesting.Claimed.is, api.tx.vesting.claim("129", "All"))
    ]).then(async function ([res1, res2, res3]) {
      claimedAmountsSchedule1 = claimedAmountsSchedule1.add(
        await verificationHandler1.verifyVestedScheduleClaim(
          SCHEDULE_AMOUNT_PER_PERIOD,
          SCHEDULE_BLOCK_PERIOD_COUNT,
          SCHEDULE_BLOCK_PERIOD,
          vestingSchedule1StartBlock,
          vestingSchedule1EndBlock,
          res1,
          "block",
          claimedAmountsSchedule1
        )
      );
      claimedAmountsSchedule2 = claimedAmountsSchedule2.add(
        await verificationHandler2.verifyVestedScheduleClaim(
          SCHEDULE_AMOUNT_PER_PERIOD,
          SCHEDULE_MOMENT_PERIOD_COUNT,
          SCHEDULE_MOMENT_PERIOD,
          vestingSchedule2StartTime,
          vestingSchedule2EndTime,
          res2,
          "moment",
          claimedAmountsSchedule2
        )
      );
      claimedAmountsSchedule3 = claimedAmountsSchedule3.add(
        await verificationHandler3.verifyVestedScheduleClaim(
          SCHEDULE_AMOUNT_PER_PERIOD,
          SCHEDULE_BLOCK_PERIOD_COUNT,
          SCHEDULE_BLOCK_PERIOD,
          vestingSchedule3StartBlock,
          vestingSchedule3EndBlock,
          res3,
          "block",
          claimedAmountsSchedule3
        )
      );
    });
  });

  it("#1.6  The beneficiary of a block based vested transfer (#1.1) can claim its transfer after the vesting period & receive the full amount.", async function () {
    this.timeout(2 * 60 * 1000);

    const assetId = PICA_ASSET_ID;
    const vestingSchedule = api.createType("ComposableTraitsVestingVestingScheduleIdSet", {
      One: api.createType("u128", vestingScheduleId1)
    });
    const wallet = wallet2;
    const schedulePeriodCount = SCHEDULE_BLOCK_PERIOD_COUNT;
    const schedulePerPeriod = SCHEDULE_AMOUNT_PER_PERIOD;

    const verificationHandler = new vestedScheduleClaimVerifier(api, wallet.publicKey, assetId, vestingScheduleId1);
    await verificationHandler.verificationSetup();

    const results = await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.vesting.Claimed.is,
      api.tx.vesting.claim(assetId, vestingSchedule)
    );

    claimedAmountsSchedule1 = claimedAmountsSchedule1.add(
      await verificationHandler.verifyVestedScheduleClaim(
        schedulePerPeriod,
        schedulePeriodCount,
        SCHEDULE_BLOCK_PERIOD,
        vestingSchedule1StartBlock,
        vestingSchedule1EndBlock,
        results,
        "block",
        claimedAmountsSchedule1
      )
    );
  });

  it("#1.8  The beneficiary of a moment based vested transfer (#1.2) can claim its transfer after the vesting period & receive the full amount.", async function () {
    this.skip();
    this.timeout(2 * 60 * 1000);

    const assetId = PICA_ASSET_ID;
    const vestingSchedule = { One: vestingScheduleId2 };
    const wallet = wallet3;
    const schedulePeriodCount = SCHEDULE_MOMENT_PERIOD_COUNT;
    const schedulePerPeriod = SCHEDULE_AMOUNT_PER_PERIOD;

    const verificationHandler = new vestedScheduleClaimVerifier(api, wallet.publicKey, assetId, vestingScheduleId2);
    await verificationHandler.verificationSetup();

    const results = await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.vesting.Claimed.is,
      api.tx.vesting.claim(assetId, vestingSchedule)
    );

    claimedAmountsSchedule2 = claimedAmountsSchedule2.add(
      await verificationHandler.verifyVestedScheduleClaim(
        schedulePerPeriod,
        schedulePeriodCount,
        SCHEDULE_MOMENT_PERIOD,
        vestingSchedule2StartTime,
        vestingSchedule2EndTime,
        results,
        "moment",
        claimedAmountsSchedule2
      )
    );
  });

  it("#1.14 A user can not claim if no funds are available for gas fees.", async function () {
    this.timeout(2 * 60 * 1000);
    const res = await sendAndWaitForSuccess(
      api,
      wallet5,
      api.events.vesting.Claimed.is,
      api.tx.vesting.claim(PICA_ASSET_ID, "All")
    ).catch(exc => {
      return exc;
    });
    expect(res.toString()).to.contain("Inability to pay some fees");
  });

  it("#1.17 A user can claim multiple vested transfer schedules for the same asset at once.", async function () {
    this.timeout(2 * 60 * 1000);

    // Providing funds to wallet.
    await mintAssetsToWallet(api, wallet5, sudoKey, [1]);
    const assetId = new BN(1);
    await sendAndWaitForSuccess(
      api,
      wallet5,
      api.events.vesting.Claimed.is,
      api.tx.vesting.claim(assetId, { Many: vestingScheduleIdCollection })
    );
  });

  it("#1.18 A user can claim all its vested transfer schedules for a given asset ID.", async function () {
    this.timeout(2 * 60 * 1000);

    const walletBeneficiary = wallet5;
    // Providing funds to wallet.
    await mintAssetsToWallet(api, walletBeneficiary, sudoKey, [1]);
    const assetId = new BN(1);
    await sendAndWaitForSuccess(
      api,
      walletBeneficiary,
      api.events.vesting.Claimed.is,
      api.tx.vesting.claim(assetId, "All")
    );
  });

  it("#1.10 I can, as sudo, update a vested transfer schedule.", async function () {
    this.timeout(2 * 60 * 1000);
    // Parameters
    const startBlock = new BN(5);
    const period = 10;
    const schedulePeriodCount = new BN(10);
    const schedulePerPeriod = new BN(10).pow(new BN(12));
    const newSchedule = [
      api.createType("ComposableTraitsVestingVestingScheduleInfo", {
        window: api.createType("ComposableTraitsVestingVestingWindow", {
          blockNumberBased: {
            start: api.createType("BlockNumber", startBlock),
            period: api.createType("BlockNumber", period)
          }
        }),
        periodCount: schedulePeriodCount,
        perPeriod: schedulePerPeriod // 1 PICA per period
      })
    ];

    const {
      data: [resultWho]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.vesting.VestingSchedulesUpdated.is,
      api.tx.sudo.sudo(api.tx.vesting.updateVestingSchedules(wallet2.publicKey, PICA_ASSET_ID, newSchedule))
    );
    expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", wallet2.publicKey).toString());
  });

  it("#1.16 I can not update a vested transfer without sudo rights.", async function () {
    this.timeout(2 * 60 * 1000);

    const startBlock = new BN(5);
    const period = 10;
    const schedulePeriodCount = new BN(10);
    const schedulePerPeriod = new BN(10).pow(new BN(12));
    const newSchedule = [
      api.createType("ComposableTraitsVestingVestingScheduleInfo", {
        window: api.createType("ComposableTraitsVestingVestingWindow", {
          blockNumberBased: {
            start: api.createType("BlockNumber", startBlock),
            period: api.createType("BlockNumber", period)
          }
        }),
        periodCount: schedulePeriodCount,
        perPeriod: schedulePerPeriod // 1 PICA per period
      })
    ];

    const res = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.vesting.VestingSchedulesUpdated.is,
      api.tx.vesting.updateVestingSchedules(wallet1.publicKey, PICA_ASSET_ID, newSchedule)
    ).catch(exc => {
      return exc;
    });
    expect(res.toString()).to.contain("BadOrigin");
  });
});
