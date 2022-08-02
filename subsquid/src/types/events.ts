import assert from 'assert'
import {EventContext, Result, deprecateLatest} from './support'
import * as v2400 from './v2400'

export class BalancesTransferEvent {
  constructor(private ctx: EventContext) {
    assert(this.ctx.event.name === 'balances.Transfer')
  }

  /**
   * Transfer succeeded.
   */
  get isV2400(): boolean {
    return this.ctx._chain.getEventHash('balances.Transfer') === '0ffdf35c495114c2d42a8bf6c241483fd5334ca0198662e14480ad040f1e3a66'
  }

  /**
   * Transfer succeeded.
   */
  get asV2400(): {from: v2400.AccountId32, to: v2400.AccountId32, amount: bigint} {
    assert(this.isV2400)
    return this.ctx._chain.decodeEvent(this.ctx.event)
  }

  get isLatest(): boolean {
    deprecateLatest()
    return this.isV2400
  }

  get asLatest(): {from: v2400.AccountId32, to: v2400.AccountId32, amount: bigint} {
    deprecateLatest()
    return this.asV2400
  }
}

export class BondedFinanceNewBondEvent {
  constructor(private ctx: EventContext) {
    assert(this.ctx.event.name === 'bondedFinance.NewBond')
  }

  /**
   * A new bond has been registered.
   */
  get isV2400(): boolean {
    return this.ctx._chain.getEventHash('bondedFinance.NewBond') === '2942193f166c2272b5592760fffb7e7332ca1fc91ea21d50ddf0a60dd35cddb7'
  }

  /**
   * A new bond has been registered.
   */
  get asV2400(): {offerId: bigint, who: v2400.AccountId32, nbOfBonds: bigint} {
    assert(this.isV2400)
    return this.ctx._chain.decodeEvent(this.ctx.event)
  }

  get isLatest(): boolean {
    deprecateLatest()
    return this.isV2400
  }

  get asLatest(): {offerId: bigint, who: v2400.AccountId32, nbOfBonds: bigint} {
    deprecateLatest()
    return this.asV2400
  }
}

export class BondedFinanceNewOfferEvent {
  constructor(private ctx: EventContext) {
    assert(this.ctx.event.name === 'bondedFinance.NewOffer')
  }

  /**
   * A new offer has been created.
   */
  get isV2400(): boolean {
    return this.ctx._chain.getEventHash('bondedFinance.NewOffer') === '68b798e0fb8f433f37ecc5a1efa5af84a146a217c123fba86d358fdc60508217'
  }

  /**
   * A new offer has been created.
   */
  get asV2400(): {offerId: bigint, beneficiary: v2400.AccountId32} {
    assert(this.isV2400)
    return this.ctx._chain.decodeEvent(this.ctx.event)
  }

  get isLatest(): boolean {
    deprecateLatest()
    return this.isV2400
  }

  get asLatest(): {offerId: bigint, beneficiary: v2400.AccountId32} {
    deprecateLatest()
    return this.asV2400
  }
}

export class PabloLiquidityAddedEvent {
  constructor(private ctx: EventContext) {
    assert(this.ctx.event.name === 'pablo.LiquidityAdded')
  }

  /**
   * Liquidity added into the pool `T::PoolId`.
   */
  get isV2400(): boolean {
    return this.ctx._chain.getEventHash('pablo.LiquidityAdded') === '312d582090ea3aa5c6ba6b929f4114d4a54ddca29cc066e4de5540c288ce5464'
  }

  /**
   * Liquidity added into the pool `T::PoolId`.
   */
  get asV2400(): {who: v2400.AccountId32, poolId: bigint, baseAmount: bigint, quoteAmount: bigint, mintedLp: bigint} {
    assert(this.isV2400)
    return this.ctx._chain.decodeEvent(this.ctx.event)
  }

  get isLatest(): boolean {
    deprecateLatest()
    return this.isV2400
  }

  get asLatest(): {who: v2400.AccountId32, poolId: bigint, baseAmount: bigint, quoteAmount: bigint, mintedLp: bigint} {
    deprecateLatest()
    return this.asV2400
  }
}

export class PabloLiquidityRemovedEvent {
  constructor(private ctx: EventContext) {
    assert(this.ctx.event.name === 'pablo.LiquidityRemoved')
  }

  /**
   * Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
   */
  get isV2400(): boolean {
    return this.ctx._chain.getEventHash('pablo.LiquidityRemoved') === 'ef123c9326de7ce47d183c1b7d729db3c90f89a6bd64122aa03a48c169c6aa5b'
  }

  /**
   * Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
   */
  get asV2400(): {who: v2400.AccountId32, poolId: bigint, baseAmount: bigint, quoteAmount: bigint, totalIssuance: bigint} {
    assert(this.isV2400)
    return this.ctx._chain.decodeEvent(this.ctx.event)
  }

  get isLatest(): boolean {
    deprecateLatest()
    return this.isV2400
  }

  get asLatest(): {who: v2400.AccountId32, poolId: bigint, baseAmount: bigint, quoteAmount: bigint, totalIssuance: bigint} {
    deprecateLatest()
    return this.asV2400
  }
}

export class PabloPoolCreatedEvent {
  constructor(private ctx: EventContext) {
    assert(this.ctx.event.name === 'pablo.PoolCreated')
  }

  /**
   * Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
   */
  get isV2400(): boolean {
    return this.ctx._chain.getEventHash('pablo.PoolCreated') === '76b660a348da63e9f657f2e6efbf072d8b02fe00cce4524df8e49986c270e996'
  }

  /**
   * Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
   */
  get asV2400(): {poolId: bigint, owner: v2400.AccountId32, assets: v2400.CurrencyPair} {
    assert(this.isV2400)
    return this.ctx._chain.decodeEvent(this.ctx.event)
  }

  get isLatest(): boolean {
    deprecateLatest()
    return this.isV2400
  }

  get asLatest(): {poolId: bigint, owner: v2400.AccountId32, assets: v2400.CurrencyPair} {
    deprecateLatest()
    return this.asV2400
  }
}

export class PabloPoolDeletedEvent {
  constructor(private ctx: EventContext) {
    assert(this.ctx.event.name === 'pablo.PoolDeleted')
  }

  /**
   * The sale ended, the funds repatriated and the pool deleted.
   */
  get isV2400(): boolean {
    return this.ctx._chain.getEventHash('pablo.PoolDeleted') === '1b2177997ab30c1eecba237f26886dc4fce241682664c0c2ccd6fa478d585089'
  }

  /**
   * The sale ended, the funds repatriated and the pool deleted.
   */
  get asV2400(): {poolId: bigint, baseAmount: bigint, quoteAmount: bigint} {
    assert(this.isV2400)
    return this.ctx._chain.decodeEvent(this.ctx.event)
  }

  get isLatest(): boolean {
    deprecateLatest()
    return this.isV2400
  }

  get asLatest(): {poolId: bigint, baseAmount: bigint, quoteAmount: bigint} {
    deprecateLatest()
    return this.asV2400
  }
}

export class PabloSwappedEvent {
  constructor(private ctx: EventContext) {
    assert(this.ctx.event.name === 'pablo.Swapped')
  }

  /**
   * Token exchange happened.
   */
  get isV2400(): boolean {
    return this.ctx._chain.getEventHash('pablo.Swapped') === 'e2cb97932583cb6d0722d9449b471d2ea8b363ac4580591664fe7471b8e463bb'
  }

  /**
   * Token exchange happened.
   */
  get asV2400(): {poolId: bigint, who: v2400.AccountId32, baseAsset: v2400.CurrencyId, quoteAsset: v2400.CurrencyId, baseAmount: bigint, quoteAmount: bigint, fee: v2400.Fee} {
    assert(this.isV2400)
    return this.ctx._chain.decodeEvent(this.ctx.event)
  }

  get isLatest(): boolean {
    deprecateLatest()
    return this.isV2400
  }

  get asLatest(): {poolId: bigint, who: v2400.AccountId32, baseAsset: v2400.CurrencyId, quoteAsset: v2400.CurrencyId, baseAmount: bigint, quoteAmount: bigint, fee: v2400.Fee} {
    deprecateLatest()
    return this.asV2400
  }
}

export class VestingVestingScheduleAddedEvent {
  constructor(private ctx: EventContext) {
    assert(this.ctx.event.name === 'vesting.VestingScheduleAdded')
  }

  /**
   * Added new vesting schedule. \[from, to, schedule\]
   */
  get isV2400(): boolean {
    return this.ctx._chain.getEventHash('vesting.VestingScheduleAdded') === 'c5e29260a72cc5736d41a9413a02519d99775ae811581363c8cbdf2433143a79'
  }

  /**
   * Added new vesting schedule. \[from, to, schedule\]
   */
  get asV2400(): {from: v2400.AccountId32, to: v2400.AccountId32, asset: v2400.CurrencyId, schedule: v2400.VestingSchedule} {
    assert(this.isV2400)
    return this.ctx._chain.decodeEvent(this.ctx.event)
  }

  get isLatest(): boolean {
    deprecateLatest()
    return this.isV2400
  }

  get asLatest(): {from: v2400.AccountId32, to: v2400.AccountId32, asset: v2400.CurrencyId, schedule: v2400.VestingSchedule} {
    deprecateLatest()
    return this.asV2400
  }
}
