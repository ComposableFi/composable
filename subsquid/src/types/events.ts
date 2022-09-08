import assert from 'assert'
import {Chain, ChainContext, EventContext, Event, Result} from './support'
import * as v2401 from './v2401'

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
  get isV2401(): boolean {
    return this._chain.getEventHash('Balances.Transfer') === '0ffdf35c495114c2d42a8bf6c241483fd5334ca0198662e14480ad040f1e3a66'
  }

  /**
   * Transfer succeeded.
   */
  get asV2401(): {from: Uint8Array, to: Uint8Array, amount: bigint} {
    assert(this.isV2401)
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
  get isV2401(): boolean {
    return this._chain.getEventHash('BondedFinance.NewBond') === '2942193f166c2272b5592760fffb7e7332ca1fc91ea21d50ddf0a60dd35cddb7'
  }

  /**
   * A new bond has been registered.
   */
  get asV2401(): {offerId: bigint, who: Uint8Array, nbOfBonds: bigint} {
    assert(this.isV2401)
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
  get isV2401(): boolean {
    return this._chain.getEventHash('BondedFinance.NewOffer') === '68b798e0fb8f433f37ecc5a1efa5af84a146a217c123fba86d358fdc60508217'
  }

  /**
   * A new offer has been created.
   */
  get asV2401(): {offerId: bigint, beneficiary: Uint8Array} {
    assert(this.isV2401)
    return this._chain.decodeEvent(this.event)
  }
}

export class PabloLiquidityAddedEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'Pablo.LiquidityAdded')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * Liquidity added into the pool `T::PoolId`.
   */
  get isV2401(): boolean {
    return this._chain.getEventHash('Pablo.LiquidityAdded') === '312d582090ea3aa5c6ba6b929f4114d4a54ddca29cc066e4de5540c288ce5464'
  }

  /**
   * Liquidity added into the pool `T::PoolId`.
   */
  get asV2401(): {who: Uint8Array, poolId: bigint, baseAmount: bigint, quoteAmount: bigint, mintedLp: bigint} {
    assert(this.isV2401)
    return this._chain.decodeEvent(this.event)
  }
}

export class PabloLiquidityRemovedEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'Pablo.LiquidityRemoved')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
   */
  get isV2401(): boolean {
    return this._chain.getEventHash('Pablo.LiquidityRemoved') === 'ef123c9326de7ce47d183c1b7d729db3c90f89a6bd64122aa03a48c169c6aa5b'
  }

  /**
   * Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
   */
  get asV2401(): {who: Uint8Array, poolId: bigint, baseAmount: bigint, quoteAmount: bigint, totalIssuance: bigint} {
    assert(this.isV2401)
    return this._chain.decodeEvent(this.event)
  }
}

export class PabloPoolCreatedEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'Pablo.PoolCreated')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
   */
  get isV2401(): boolean {
    return this._chain.getEventHash('Pablo.PoolCreated') === '76b660a348da63e9f657f2e6efbf072d8b02fe00cce4524df8e49986c270e996'
  }

  /**
   * Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
   */
  get asV2401(): {poolId: bigint, owner: Uint8Array, assets: v2401.CurrencyPair} {
    assert(this.isV2401)
    return this._chain.decodeEvent(this.event)
  }
}

export class PabloPoolDeletedEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'Pablo.PoolDeleted')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * The sale ended, the funds repatriated and the pool deleted.
   */
  get isV2401(): boolean {
    return this._chain.getEventHash('Pablo.PoolDeleted') === '1b2177997ab30c1eecba237f26886dc4fce241682664c0c2ccd6fa478d585089'
  }

  /**
   * The sale ended, the funds repatriated and the pool deleted.
   */
  get asV2401(): {poolId: bigint, baseAmount: bigint, quoteAmount: bigint} {
    assert(this.isV2401)
    return this._chain.decodeEvent(this.event)
  }
}

export class PabloSwappedEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'Pablo.Swapped')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * Token exchange happened.
   */
  get isV2401(): boolean {
    return this._chain.getEventHash('Pablo.Swapped') === 'e2cb97932583cb6d0722d9449b471d2ea8b363ac4580591664fe7471b8e463bb'
  }

  /**
   * Token exchange happened.
   */
  get asV2401(): {poolId: bigint, who: Uint8Array, baseAsset: bigint, quoteAsset: bigint, baseAmount: bigint, quoteAmount: bigint, fee: v2401.Fee} {
    assert(this.isV2401)
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
   * Added new vesting schedule.
   */
  get isV2401(): boolean {
    return this._chain.getEventHash('Vesting.VestingScheduleAdded') === '76bb06af4efc9a40f5604bfe9dbe980d1cec79e966fe1f641bb9475c65a6808d'
  }

  /**
   * Added new vesting schedule.
   */
  get asV2401(): {from: Uint8Array, to: Uint8Array, asset: bigint, vestingScheduleId: bigint, schedule: v2401.VestingSchedule, scheduleAmount: bigint} {
    assert(this.isV2401)
    return this._chain.decodeEvent(this.event)
  }
}
