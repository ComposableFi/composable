import assert from 'assert'
import {Chain, ChainContext, EventContext, Event, Result} from './support'
import * as v1000 from './v1000'
import * as v1200 from './v1200'
import * as v10002 from './v10002'

export class AssetsRegistryAssetRegisteredEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'AssetsRegistry.AssetRegistered')
    this._chain = ctx._chain
    this.event = event
  }

  get isV1200(): boolean {
    return this._chain.getEventHash('AssetsRegistry.AssetRegistered') === 'ce8c5bb44dcb9f35892515385bfbeb519c75c883b4e1facffafc657f308f73ae'
  }

  get asV1200(): {assetId: bigint, location: v1200.XcmAssetLocation} {
    assert(this.isV1200)
    return this._chain.decodeEvent(this.event)
  }

  get isV10002(): boolean {
    return this._chain.getEventHash('AssetsRegistry.AssetRegistered') === '1a2df5dc03da9f3f2850f9f87cba28c55a399bcfa2b16bfd3911fb85c1653654'
  }

  get asV10002(): {assetId: bigint, location: v10002.XcmAssetLocation, decimals: (number | undefined)} {
    assert(this.isV10002)
    return this._chain.decodeEvent(this.event)
  }
}

export class AssetsRegistryAssetUpdatedEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'AssetsRegistry.AssetUpdated')
    this._chain = ctx._chain
    this.event = event
  }

  get isV1200(): boolean {
    return this._chain.getEventHash('AssetsRegistry.AssetUpdated') === 'ce8c5bb44dcb9f35892515385bfbeb519c75c883b4e1facffafc657f308f73ae'
  }

  get asV1200(): {assetId: bigint, location: v1200.XcmAssetLocation} {
    assert(this.isV1200)
    return this._chain.decodeEvent(this.event)
  }

  get isV10002(): boolean {
    return this._chain.getEventHash('AssetsRegistry.AssetUpdated') === '1a2df5dc03da9f3f2850f9f87cba28c55a399bcfa2b16bfd3911fb85c1653654'
  }

  get asV10002(): {assetId: bigint, location: v10002.XcmAssetLocation, decimals: (number | undefined)} {
    assert(this.isV10002)
    return this._chain.decodeEvent(this.event)
  }
}

export class BalancesDepositEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'Balances.Deposit')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * Some amount was deposited (e.g. for transaction fees).
   */
  get isV200(): boolean {
    return this._chain.getEventHash('Balances.Deposit') === 'e84a34a6a3d577b31f16557bd304282f4fe4cbd7115377f4687635dc48e52ba5'
  }

  /**
   * Some amount was deposited (e.g. for transaction fees).
   */
  get asV200(): {who: Uint8Array, amount: bigint} {
    assert(this.isV200)
    return this._chain.decodeEvent(this.event)
  }
}

export class BalancesSlashedEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'Balances.Slashed')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * Some amount was removed from the account (e.g. for misbehavior).
   */
  get isV200(): boolean {
    return this._chain.getEventHash('Balances.Slashed') === 'e84a34a6a3d577b31f16557bd304282f4fe4cbd7115377f4687635dc48e52ba5'
  }

  /**
   * Some amount was removed from the account (e.g. for misbehavior).
   */
  get asV200(): {who: Uint8Array, amount: bigint} {
    assert(this.isV200)
    return this._chain.decodeEvent(this.event)
  }
}

export class BalancesTransferEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'Balances.Transfer')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * Transfer succeeded.
   */
  get isV200(): boolean {
    return this._chain.getEventHash('Balances.Transfer') === '0ffdf35c495114c2d42a8bf6c241483fd5334ca0198662e14480ad040f1e3a66'
  }

  /**
   * Transfer succeeded.
   */
  get asV200(): {from: Uint8Array, to: Uint8Array, amount: bigint} {
    assert(this.isV200)
    return this._chain.decodeEvent(this.event)
  }
}

export class BalancesWithdrawEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'Balances.Withdraw')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * Some amount was withdrawn from the account (e.g. for transaction fees).
   */
  get isV200(): boolean {
    return this._chain.getEventHash('Balances.Withdraw') === 'e84a34a6a3d577b31f16557bd304282f4fe4cbd7115377f4687635dc48e52ba5'
  }

  /**
   * Some amount was withdrawn from the account (e.g. for transaction fees).
   */
  get asV200(): {who: Uint8Array, amount: bigint} {
    assert(this.isV200)
    return this._chain.decodeEvent(this.event)
  }
}

export class BondedFinanceNewBondEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'BondedFinance.NewBond')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * A new bond has been registered.
   */
  get isV1000(): boolean {
    return this._chain.getEventHash('BondedFinance.NewBond') === '2942193f166c2272b5592760fffb7e7332ca1fc91ea21d50ddf0a60dd35cddb7'
  }

  /**
   * A new bond has been registered.
   */
  get asV1000(): {offerId: bigint, who: Uint8Array, nbOfBonds: bigint} {
    assert(this.isV1000)
    return this._chain.decodeEvent(this.event)
  }
}

export class BondedFinanceNewOfferEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'BondedFinance.NewOffer')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * A new offer has been created.
   */
  get isV1000(): boolean {
    return this._chain.getEventHash('BondedFinance.NewOffer') === 'a31df34b423037e305dbc2946d691428051e98fb362268dc0e78aff52ab30840'
  }

  /**
   * A new offer has been created.
   */
  get asV1000(): {offerId: bigint} {
    assert(this.isV1000)
    return this._chain.decodeEvent(this.event)
  }

  /**
   * A new offer has been created.
   */
  get isV1400(): boolean {
    return this._chain.getEventHash('BondedFinance.NewOffer') === '68b798e0fb8f433f37ecc5a1efa5af84a146a217c123fba86d358fdc60508217'
  }

  /**
   * A new offer has been created.
   */
  get asV1400(): {offerId: bigint, beneficiary: Uint8Array} {
    assert(this.isV1400)
    return this._chain.decodeEvent(this.event)
  }
}

export class BondedFinanceOfferCancelledEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'BondedFinance.OfferCancelled')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * An offer has been cancelled by the `AdminOrigin`.
   */
  get isV1000(): boolean {
    return this._chain.getEventHash('BondedFinance.OfferCancelled') === 'a31df34b423037e305dbc2946d691428051e98fb362268dc0e78aff52ab30840'
  }

  /**
   * An offer has been cancelled by the `AdminOrigin`.
   */
  get asV1000(): {offerId: bigint} {
    assert(this.isV1000)
    return this._chain.decodeEvent(this.event)
  }
}

export class VestingClaimedEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'Vesting.Claimed')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * Claimed vesting. \[who, locked_amount\]
   */
  get isV1000(): boolean {
    return this._chain.getEventHash('Vesting.Claimed') === '1f29af233c75b3b7d43d3ffbfe7da109a4f7c9f277896999fac76012939a6432'
  }

  /**
   * Claimed vesting. \[who, locked_amount\]
   */
  get asV1000(): {who: Uint8Array, asset: bigint, lockedAmount: bigint} {
    assert(this.isV1000)
    return this._chain.decodeEvent(this.event)
  }

  /**
   * Claimed vesting.
   */
  get isV10002(): boolean {
    return this._chain.getEventHash('Vesting.Claimed') === '1158bd677eb4e5aad57841bad2e35470c5be3bbc33b843378d69a8cf7bfced30'
  }

  /**
   * Claimed vesting.
   */
  get asV10002(): {who: Uint8Array, asset: bigint, vestingScheduleIds: v10002.VestingScheduleIdSet, lockedAmount: bigint, claimedAmountPerSchedule: [bigint, bigint][]} {
    assert(this.isV10002)
    return this._chain.decodeEvent(this.event)
  }
}

export class VestingVestingScheduleAddedEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'Vesting.VestingScheduleAdded')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * Added new vesting schedule. \[from, to, schedule\]
   */
  get isV1000(): boolean {
    return this._chain.getEventHash('Vesting.VestingScheduleAdded') === 'c5e29260a72cc5736d41a9413a02519d99775ae811581363c8cbdf2433143a79'
  }

  /**
   * Added new vesting schedule. \[from, to, schedule\]
   */
  get asV1000(): {from: Uint8Array, to: Uint8Array, asset: bigint, schedule: v1000.VestingSchedule} {
    assert(this.isV1000)
    return this._chain.decodeEvent(this.event)
  }

  /**
   * Added new vesting schedule.
   */
  get isV10002(): boolean {
    return this._chain.getEventHash('Vesting.VestingScheduleAdded') === '76bb06af4efc9a40f5604bfe9dbe980d1cec79e966fe1f641bb9475c65a6808d'
  }

  /**
   * Added new vesting schedule.
   */
  get asV10002(): {from: Uint8Array, to: Uint8Array, asset: bigint, vestingScheduleId: bigint, schedule: v10002.VestingSchedule, scheduleAmount: bigint} {
    assert(this.isV10002)
    return this._chain.decodeEvent(this.event)
  }
}

export class VestingVestingSchedulesUpdatedEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'Vesting.VestingSchedulesUpdated')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * Updated vesting schedules. \[who\]
   */
  get isV1000(): boolean {
    return this._chain.getEventHash('Vesting.VestingSchedulesUpdated') === 'b8a0d2208835f6ada60dd21cd93533d703777b3779109a7c6a2f26bad68c2f3b'
  }

  /**
   * Updated vesting schedules. \[who\]
   */
  get asV1000(): {who: Uint8Array} {
    assert(this.isV1000)
    return this._chain.decodeEvent(this.event)
  }
}
