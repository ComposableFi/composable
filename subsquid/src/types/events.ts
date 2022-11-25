import assert from 'assert'
import {Chain, ChainContext, EventContext, Event, Result} from './support'
import * as v2402 from './v2402'

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

  get isV2402(): boolean {
    return this._chain.getEventHash('AssetsRegistry.AssetRegistered') === '1a2df5dc03da9f3f2850f9f87cba28c55a399bcfa2b16bfd3911fb85c1653654'
  }

  get asV2402(): {assetId: bigint, location: v2402.XcmAssetLocation, decimals: (number | undefined)} {
    assert(this.isV2402)
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

  get isV2402(): boolean {
    return this._chain.getEventHash('AssetsRegistry.AssetUpdated') === '1a2df5dc03da9f3f2850f9f87cba28c55a399bcfa2b16bfd3911fb85c1653654'
  }

  get asV2402(): {assetId: bigint, location: v2402.XcmAssetLocation, decimals: (number | undefined)} {
    assert(this.isV2402)
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
  get isV2402(): boolean {
    return this._chain.getEventHash('Balances.Deposit') === 'e84a34a6a3d577b31f16557bd304282f4fe4cbd7115377f4687635dc48e52ba5'
  }

  /**
   * Some amount was deposited (e.g. for transaction fees).
   */
  get asV2402(): {who: Uint8Array, amount: bigint} {
    assert(this.isV2402)
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
  get isV2402(): boolean {
    return this._chain.getEventHash('Balances.Slashed') === 'e84a34a6a3d577b31f16557bd304282f4fe4cbd7115377f4687635dc48e52ba5'
  }

  /**
   * Some amount was removed from the account (e.g. for misbehavior).
   */
  get asV2402(): {who: Uint8Array, amount: bigint} {
    assert(this.isV2402)
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
  get isV2402(): boolean {
    return this._chain.getEventHash('Balances.Transfer') === '0ffdf35c495114c2d42a8bf6c241483fd5334ca0198662e14480ad040f1e3a66'
  }

  /**
   * Transfer succeeded.
   */
  get asV2402(): {from: Uint8Array, to: Uint8Array, amount: bigint} {
    assert(this.isV2402)
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
  get isV2402(): boolean {
    return this._chain.getEventHash('Balances.Withdraw') === 'e84a34a6a3d577b31f16557bd304282f4fe4cbd7115377f4687635dc48e52ba5'
  }

  /**
   * Some amount was withdrawn from the account (e.g. for transaction fees).
   */
  get asV2402(): {who: Uint8Array, amount: bigint} {
    assert(this.isV2402)
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
  get isV2402(): boolean {
    return this._chain.getEventHash('BondedFinance.NewBond') === '2942193f166c2272b5592760fffb7e7332ca1fc91ea21d50ddf0a60dd35cddb7'
  }

  /**
   * A new bond has been registered.
   */
  get asV2402(): {offerId: bigint, who: Uint8Array, nbOfBonds: bigint} {
    assert(this.isV2402)
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
  get isV2402(): boolean {
    return this._chain.getEventHash('BondedFinance.NewOffer') === '68b798e0fb8f433f37ecc5a1efa5af84a146a217c123fba86d358fdc60508217'
  }

  /**
   * A new offer has been created.
   */
  get asV2402(): {offerId: bigint, beneficiary: Uint8Array} {
    assert(this.isV2402)
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
  get isV2402(): boolean {
    return this._chain.getEventHash('BondedFinance.OfferCancelled') === 'a31df34b423037e305dbc2946d691428051e98fb362268dc0e78aff52ab30840'
  }

  /**
   * An offer has been cancelled by the `AdminOrigin`.
   */
  get asV2402(): {offerId: bigint} {
    assert(this.isV2402)
    return this._chain.decodeEvent(this.event)
  }
}

export class OraclePriceChangedEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'Oracle.PriceChanged')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * Price changed by oracle \[asset_id, price\]
   */
  get isV2402(): boolean {
    return this._chain.getEventHash('Oracle.PriceChanged') === 'f7d5bd1431cb954502149f64a8137986d660e0729a3d9731d421496b4298be52'
  }

  /**
   * Price changed by oracle \[asset_id, price\]
   */
  get asV2402(): [bigint, bigint] {
    assert(this.isV2402)
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
  get isV2402(): boolean {
    return this._chain.getEventHash('Pablo.LiquidityAdded') === '486edbf8643ab771073bd483f4953899fa28a13f3c282666a2462c3cca2570c7'
  }

  /**
   * Liquidity added into the pool `T::PoolId`.
   */
  get asV2402(): {who: Uint8Array, poolId: bigint, assets: [bigint, bigint][], mintedLp: bigint} {
    assert(this.isV2402)
    return this._chain.decodeEvent(this.event)
  }

  /**
   * Liquidity added into the pool `T::PoolId`.
   */
  get isV10002(): boolean {
    return this._chain.getEventHash('Pablo.LiquidityAdded') === '312d582090ea3aa5c6ba6b929f4114d4a54ddca29cc066e4de5540c288ce5464'
  }

  /**
   * Liquidity added into the pool `T::PoolId`.
   */
  get asV10002(): {who: Uint8Array, poolId: bigint, baseAmount: bigint, quoteAmount: bigint, mintedLp: bigint} {
    assert(this.isV10002)
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
  get isV2402(): boolean {
    return this._chain.getEventHash('Pablo.LiquidityRemoved') === '2c7e841374438a598134480f43b6c2b079b50bd25457174f647f6dca94a93d53'
  }

  /**
   * Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
   */
  get asV2402(): {who: Uint8Array, poolId: bigint, assets: [bigint, bigint][], totalIssuance: bigint} {
    assert(this.isV2402)
    return this._chain.decodeEvent(this.event)
  }

  /**
   * Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
   */
  get isV10002(): boolean {
    return this._chain.getEventHash('Pablo.LiquidityRemoved') === 'ef123c9326de7ce47d183c1b7d729db3c90f89a6bd64122aa03a48c169c6aa5b'
  }

  /**
   * Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
   */
  get asV10002(): {who: Uint8Array, poolId: bigint, baseAmount: bigint, quoteAmount: bigint, totalIssuance: bigint} {
    assert(this.isV10002)
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
  get isV2402(): boolean {
    return this._chain.getEventHash('Pablo.PoolCreated') === '76b660a348da63e9f657f2e6efbf072d8b02fe00cce4524df8e49986c270e996'
  }

  /**
   * Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
   */
  get asV2402(): {poolId: bigint, owner: Uint8Array, assets: v2402.CurrencyPair} {
    assert(this.isV2402)
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
  get isV2402(): boolean {
    return this._chain.getEventHash('Pablo.PoolDeleted') === '19ca4533e16e3838ae98776a6f38c75647996f376f07e614f658d78acd1c3f65'
  }

  /**
   * The sale ended, the funds repatriated and the pool deleted.
   */
  get asV2402(): {poolId: bigint, assets: [bigint, bigint][]} {
    assert(this.isV2402)
    return this._chain.decodeEvent(this.event)
  }

  /**
   * The sale ended, the funds repatriated and the pool deleted.
   */
  get isV10002(): boolean {
    return this._chain.getEventHash('Pablo.PoolDeleted') === '1b2177997ab30c1eecba237f26886dc4fce241682664c0c2ccd6fa478d585089'
  }

  /**
   * The sale ended, the funds repatriated and the pool deleted.
   */
  get asV10002(): {poolId: bigint, baseAmount: bigint, quoteAmount: bigint} {
    assert(this.isV10002)
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
  get isV2402(): boolean {
    return this._chain.getEventHash('Pablo.Swapped') === 'e2cb97932583cb6d0722d9449b471d2ea8b363ac4580591664fe7471b8e463bb'
  }

  /**
   * Token exchange happened.
   */
  get asV2402(): {poolId: bigint, who: Uint8Array, baseAsset: bigint, quoteAsset: bigint, baseAmount: bigint, quoteAmount: bigint, fee: v2402.Fee} {
    assert(this.isV2402)
    return this._chain.decodeEvent(this.event)
  }
}

export class StakingRewardsRewardPoolCreatedEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'StakingRewards.RewardPoolCreated')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * Pool with specified id `T::AssetId` was created successfully by `T::AccountId`.
   */
  get isV2402(): boolean {
    return this._chain.getEventHash('StakingRewards.RewardPoolCreated') === '7297483572d80cd839ccdcfe9189950b6ea9fe87ab6130a1611627cebbfb2b0d'
  }

  /**
   * Pool with specified id `T::AssetId` was created successfully by `T::AccountId`.
   */
  get asV2402(): {poolId: bigint, owner: Uint8Array, endBlock: number} {
    assert(this.isV2402)
    return this._chain.decodeEvent(this.event)
  }
}

export class StakingRewardsSplitPositionEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'StakingRewards.SplitPosition')
    this._chain = ctx._chain
    this.event = event
  }

  /**
   * A staking position was split.
   */
  get isV2402(): boolean {
    return this._chain.getEventHash('StakingRewards.SplitPosition') === '8031e5788ebd4aef1e0b75ca8f52827e9667c64d25433e9e070ed74ba3f9a8e3'
  }

  /**
   * A staking position was split.
   */
  get asV2402(): {positions: [bigint, bigint, bigint][]} {
    assert(this.isV2402)
    return this._chain.decodeEvent(this.event)
  }
}

export class StakingRewardsStakeAmountExtendedEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'StakingRewards.StakeAmountExtended')
    this._chain = ctx._chain
    this.event = event
  }

  get isV2402(): boolean {
    return this._chain.getEventHash('StakingRewards.StakeAmountExtended') === 'a41f82bf4e9ef7f7f630f7fb5696e3b2f4ca4baf7eb8af6a70d3faf535de3dc9'
  }

  get asV2402(): {fnftCollectionId: bigint, fnftInstanceId: bigint, amount: bigint} {
    assert(this.isV2402)
    return this._chain.decodeEvent(this.event)
  }
}

export class StakingRewardsStakedEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'StakingRewards.Staked')
    this._chain = ctx._chain
    this.event = event
  }

  get isV2402(): boolean {
    return this._chain.getEventHash('StakingRewards.Staked') === '51b801cf0907ec7517c5d002143ad3b27067de4f35b313be7a019444cfb926e0'
  }

  get asV2402(): {poolId: bigint, owner: Uint8Array, amount: bigint, durationPreset: bigint, fnftCollectionId: bigint, fnftInstanceId: bigint, rewardMultiplier: bigint, keepAlive: boolean} {
    assert(this.isV2402)
    return this._chain.decodeEvent(this.event)
  }
}

export class StakingRewardsUnstakedEvent {
  private readonly _chain: Chain
  private readonly event: Event

  constructor(ctx: EventContext)
  constructor(ctx: ChainContext, event: Event)
  constructor(ctx: EventContext, event?: Event) {
    event = event || ctx.event
    assert(event.name === 'StakingRewards.Unstaked')
    this._chain = ctx._chain
    this.event = event
  }

  get isV2402(): boolean {
    return this._chain.getEventHash('StakingRewards.Unstaked') === '49170892b4bc964636d4793ef5cf7eee1abd5fd1f34f55f4448294ba4a184c73'
  }

  get asV2402(): {owner: Uint8Array, fnftCollectionId: bigint, fnftInstanceId: bigint, slash: (bigint | undefined)} {
    assert(this.isV2402)
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
   * Claimed vesting.
   */
  get isV2402(): boolean {
    return this._chain.getEventHash('Vesting.Claimed') === '1158bd677eb4e5aad57841bad2e35470c5be3bbc33b843378d69a8cf7bfced30'
  }

  /**
   * Claimed vesting.
   */
  get asV2402(): {who: Uint8Array, asset: bigint, vestingScheduleIds: v2402.VestingScheduleIdSet, lockedAmount: bigint, claimedAmountPerSchedule: [bigint, bigint][]} {
    assert(this.isV2402)
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
  get isV2402(): boolean {
    return this._chain.getEventHash('Vesting.VestingScheduleAdded') === '76bb06af4efc9a40f5604bfe9dbe980d1cec79e966fe1f641bb9475c65a6808d'
  }

  /**
   * Added new vesting schedule.
   */
  get asV2402(): {from: Uint8Array, to: Uint8Array, asset: bigint, vestingScheduleId: bigint, schedule: v2402.VestingSchedule, scheduleAmount: bigint} {
    assert(this.isV2402)
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
   * Updated vesting schedules.
   */
  get isV2402(): boolean {
    return this._chain.getEventHash('Vesting.VestingSchedulesUpdated') === 'b8a0d2208835f6ada60dd21cd93533d703777b3779109a7c6a2f26bad68c2f3b'
  }

  /**
   * Updated vesting schedules.
   */
  get asV2402(): {who: Uint8Array} {
    assert(this.isV2402)
    return this._chain.decodeEvent(this.event)
  }
}
