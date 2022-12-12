import BN from "bn.js";
import { ApiPromise } from "@polkadot/api";
import { AssertionError, expect } from "chai";
import {
  ComposableTraitsVestingVestingSchedule,
  ComposableTraitsVestingVestingScheduleIdSet,
  ComposableTraitsVestingVestingScheduleInfo,
  CustomRpcBalance,
  FrameSystemAccountInfo,
  OrmlTokensAccountData
} from "@composable/types/interfaces";
import { AnyNumber, IEvent, IEventData } from "@polkadot/types/types";
import { AccountId32 } from "@polkadot/types/interfaces";
import { BTreeMap, u128 } from "@polkadot/types-codec";

export function decodeVestingSchedule(vestingSchedule: any) {
  const type = vestingSchedule.window.blockNumberBased ? "block" : "moment";
  const window = {
    start: vestingSchedule.window.blockNumberBased
      ? new BN(vestingSchedule.window.blockNumberBased.start)
      : new BN(vestingSchedule.window.momentBased.start),
    period: vestingSchedule.window.blockNumberBased
      ? new BN(vestingSchedule.window.blockNumberBased.period)
      : new BN(vestingSchedule.window.momentBased.period)
  };
  return {
    perPeriod: vestingSchedule.perPeriod,
    periodCount: Number(vestingSchedule.periodCount),
    window,
    type,
    alreadyClaimed: vestingSchedule.alreadyClaimed,
    vestingScheduleId: new BN(vestingSchedule.vestingScheduleId)
  };
}

export class vestedScheduleCreationVerifier {
  walletBalanceBefore: CustomRpcBalance | undefined;
  api: ApiPromise;

  constructor(api: ApiPromise) {
    this.api = api;
  }

  async verificationSetup(assetId: BN | number | AnyNumber, senderWallet: string | Uint8Array) {
    this.walletBalanceBefore = await this.api.rpc.assets.balanceOf(assetId.toString(), senderWallet);
  }

  async verifyVestedScheduleCreation(
    senderWallet: string | Uint8Array,
    beneficiaryWallet: string | Uint8Array,
    assetId: u128 | BN,
    schedulePerPeriod: BN,
    schedulePeriodCount: BN,
    scheduleInfo: ComposableTraitsVestingVestingScheduleInfo,
    resultData: [
      from: AccountId32,
      to: AccountId32,
      asset: u128,
      vestingScheduleId: u128,
      schedule: ComposableTraitsVestingVestingSchedule,
      scheduleAmount: u128
    ] &
      IEventData,
    vestingMethod: "moment" | "block"
  ) {
    if (!this.walletBalanceBefore)
      throw new AssertionError("Couldn't verify vesting schedule!\nWallet balance before transaction wasn't defined.");
    const [resultFrom, resultTo, resultAssetId, , resultSchedule] = resultData;
    expect(resultFrom.toString()).to.be.equal(this.api.createType("AccountId32", senderWallet).toString());
    expect(resultTo.toString()).to.be.equal(this.api.createType("AccountId32", beneficiaryWallet).toString());
    expect(resultAssetId).to.be.bignumber.equal(new BN(assetId));

    const vestingScheduleQuery = await this.api.query.vesting.vestingSchedules(beneficiaryWallet, assetId);
    const _schedules = vestingScheduleQuery.toJSON();
    const schedules = Object.values(_schedules as any).map(i => decodeVestingSchedule(i));

    const vestingScheduleIndex = vestingScheduleQuery.size - 1;

    if (vestingMethod === "moment") {
      expect(resultSchedule.window.asMomentBased.start.toString())
        .to.be.eql(scheduleInfo.window.asMomentBased.start.toString())
        .to.be.eql(schedules[vestingScheduleIndex].window.start.toString());
      expect(resultSchedule.window.asMomentBased.period.toString())
        .to.be.eql(scheduleInfo.window.asMomentBased.period.toString())
        .to.be.eql(schedules[vestingScheduleIndex].window.period.toString());
    } else {
      expect(resultSchedule.window.asBlockNumberBased.start.toString())
        .to.be.eql(scheduleInfo.window.asBlockNumberBased.start.toString())
        .to.be.eql(schedules[vestingScheduleIndex].window.start.toString());
      expect(resultSchedule.window.asBlockNumberBased.period.toString())
        .to.be.eql(scheduleInfo.window.asBlockNumberBased.period.toString())
        .to.be.eql(schedules[vestingScheduleIndex].window.period.toString());
    }
    expect(resultSchedule.periodCount.toString())
      .to.be.equal(schedulePeriodCount.toString())
      .to.be.equal(schedules[vestingScheduleIndex].periodCount.toString());
    expect(resultSchedule.perPeriod.toString())
      .to.be.equal(schedulePerPeriod.toString())
      .to.be.equal(schedules[vestingScheduleIndex].perPeriod.toString());

    // Verifying balances
    const walletBalanceAfter = await this.api.rpc.assets.balanceOf(assetId.toString(), senderWallet);
    expect(new BN(walletBalanceAfter.toString())).to.be.bignumber.equal(
      new BN(this.walletBalanceBefore.toString()).sub(schedulePerPeriod.mul(schedulePeriodCount))
    );
  }
}

export class vestedScheduleClaimVerifier {
  api: ApiPromise;
  beneficiaryPublicKey: string | Uint8Array;
  walletBalanceBeneficiaryBefore: BN | undefined;
  vestingScheduleId: BN;

  assetId: BN;

  constructor(api: ApiPromise, walletBeneficiary: string | Uint8Array, assetId: BN | number, vestingScheduleId: BN) {
    this.api = api;
    this.beneficiaryPublicKey = walletBeneficiary;
    this.assetId = new BN(assetId.toString());
    this.vestingScheduleId = vestingScheduleId;
  }

  async verificationSetup() {
    if (this.assetId.eq(new BN(1))) {
      const accountInfo = <FrameSystemAccountInfo>await this.api.query.system.account(this.beneficiaryPublicKey);
      this.walletBalanceBeneficiaryBefore = new BN(accountInfo.data.free.sub(accountInfo.data.miscFrozen).toString());
    } else {
      const tokenAccountInfo = <OrmlTokensAccountData>(
        await this.api.query.tokens.accounts(this.beneficiaryPublicKey, this.assetId)
      );
      this.walletBalanceBeneficiaryBefore = tokenAccountInfo.free.sub(tokenAccountInfo.frozen);
    }
  }

  async verifyVestedScheduleClaim(
    schedulePerPeriod: BN,
    schedulePeriodCount: BN,
    schedulePeriodLength: BN,
    start: BN,
    end: BN,
    resultData: IEvent<
      [
        who: AccountId32,
        asset: u128,
        vestingScheduleIds: ComposableTraitsVestingVestingScheduleIdSet,
        lockedAmount: u128,
        claimedAmountPerSchedule: BTreeMap<u128, u128>
      ]
    >,
    vestingMethod: "moment" | "block",
    alreadyClaimed = new BN(0)
  ) {
    if (!this.walletBalanceBeneficiaryBefore || !this.beneficiaryPublicKey)
      throw new AssertionError(
        "Couldn't verify vesting schedule!\nWallet or wallet balance before transaction wasn't defined."
      );

    let expectedClaimAmount;
    if (vestingMethod === "moment") {
      // Verifying moment based vested schedule claim
      const currentTime = new BN(Date.now());
      const pastSinceStart = BN.min(currentTime.sub(start), end);
      expectedClaimAmount = schedulePerPeriod
        .mul(pastSinceStart.div(schedulePeriodLength))
      if (!expectedClaimAmount.eqn(0)) expectedClaimAmount = expectedClaimAmount.sub(schedulePerPeriod)
    } else {
      // Verifying block number based vested schedule claim
      const currentBlock = await this.api.query.system.number();
      const pastSinceStart = BN.min(currentBlock.sub(start), end);
      expectedClaimAmount = schedulePerPeriod.mul(pastSinceStart.div(schedulePeriodLength));
    }
    expectedClaimAmount = expectedClaimAmount.sub(alreadyClaimed);

    let walletBalanceBeneficiaryAfter;
    if (this.assetId.eq(new BN(1))) {
      const accountInfo = <FrameSystemAccountInfo>await this.api.query.system.account(this.beneficiaryPublicKey);
      walletBalanceBeneficiaryAfter = new BN(accountInfo.data.free.sub(accountInfo.data.miscFrozen).toString());
    } else {
      const tokenAccountInfo = <OrmlTokensAccountData>(
        await this.api.query.tokens.accounts(this.beneficiaryPublicKey, this.assetId)
      );
      walletBalanceBeneficiaryAfter = tokenAccountInfo.free.sub(tokenAccountInfo.frozen);
    }
    let exactAmount = new BN(0);
    for (const val of resultData.data[4]) {
      // For some reason indexing 0 didn't work therefore unnecessary looping.
      exactAmount = exactAmount.add(val[1]);
    }
    // @ts-ignore
    if (!exactAmount) throw new AssertionError("No exact amount determined!");
    expect(exactAmount).to.be.bignumber.equal(expectedClaimAmount);
    expect(walletBalanceBeneficiaryAfter).to.be.bignumber.closeTo(
      this.walletBalanceBeneficiaryBefore.add(exactAmount),
      new BN(200_000_000_000)
    );
    return exactAmount;
  }
}
