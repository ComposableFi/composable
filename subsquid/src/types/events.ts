import assert from 'assert'
import {Chain, ChainContext, EventContext, Event, Result, Option} from './support'
import * as v10005 from './v10005'
import * as picassoV1000 from './picassoV1000'
import * as picassoV1200 from './picassoV1200'
import * as picassoV10002 from './picassoV10002'
import * as picassoV10005 from './picassoV10005'

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

    get isPicassoV1200(): boolean {
        return this._chain.getEventHash('AssetsRegistry.AssetRegistered') === 'ce8c5bb44dcb9f35892515385bfbeb519c75c883b4e1facffafc657f308f73ae'
    }

    get asPicassoV1200(): {assetId: bigint, location: picassoV1200.XcmAssetLocation} {
        assert(this.isPicassoV1200)
        return this._chain.decodeEvent(this.event)
    }

    get isPicassoV10002(): boolean {
        return this._chain.getEventHash('AssetsRegistry.AssetRegistered') === '1a2df5dc03da9f3f2850f9f87cba28c55a399bcfa2b16bfd3911fb85c1653654'
    }

    get asPicassoV10002(): {assetId: bigint, location: picassoV10002.XcmAssetLocation, decimals: (number | undefined)} {
        assert(this.isPicassoV10002)
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

    get isPicassoV1200(): boolean {
        return this._chain.getEventHash('AssetsRegistry.AssetUpdated') === 'ce8c5bb44dcb9f35892515385bfbeb519c75c883b4e1facffafc657f308f73ae'
    }

    get asPicassoV1200(): {assetId: bigint, location: picassoV1200.XcmAssetLocation} {
        assert(this.isPicassoV1200)
        return this._chain.decodeEvent(this.event)
    }

    get isPicassoV10002(): boolean {
        return this._chain.getEventHash('AssetsRegistry.AssetUpdated') === '1a2df5dc03da9f3f2850f9f87cba28c55a399bcfa2b16bfd3911fb85c1653654'
    }

    get asPicassoV10002(): {assetId: bigint, location: picassoV10002.XcmAssetLocation, decimals: (number | undefined)} {
        assert(this.isPicassoV10002)
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
    get isPicassoV200(): boolean {
        return this._chain.getEventHash('Balances.Deposit') === 'e84a34a6a3d577b31f16557bd304282f4fe4cbd7115377f4687635dc48e52ba5'
    }

    /**
     * Some amount was deposited (e.g. for transaction fees).
     */
    get asPicassoV200(): {who: Uint8Array, amount: bigint} {
        assert(this.isPicassoV200)
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
    get isPicassoV200(): boolean {
        return this._chain.getEventHash('Balances.Slashed') === 'e84a34a6a3d577b31f16557bd304282f4fe4cbd7115377f4687635dc48e52ba5'
    }

    /**
     * Some amount was removed from the account (e.g. for misbehavior).
     */
    get asPicassoV200(): {who: Uint8Array, amount: bigint} {
        assert(this.isPicassoV200)
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
    get isPicassoV200(): boolean {
        return this._chain.getEventHash('Balances.Transfer') === '0ffdf35c495114c2d42a8bf6c241483fd5334ca0198662e14480ad040f1e3a66'
    }

    /**
     * Transfer succeeded.
     */
    get asPicassoV200(): {from: Uint8Array, to: Uint8Array, amount: bigint} {
        assert(this.isPicassoV200)
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
    get isPicassoV200(): boolean {
        return this._chain.getEventHash('Balances.Withdraw') === 'e84a34a6a3d577b31f16557bd304282f4fe4cbd7115377f4687635dc48e52ba5'
    }

    /**
     * Some amount was withdrawn from the account (e.g. for transaction fees).
     */
    get asPicassoV200(): {who: Uint8Array, amount: bigint} {
        assert(this.isPicassoV200)
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
    get isPicassoV1000(): boolean {
        return this._chain.getEventHash('BondedFinance.NewBond') === '2942193f166c2272b5592760fffb7e7332ca1fc91ea21d50ddf0a60dd35cddb7'
    }

    /**
     * A new bond has been registered.
     */
    get asPicassoV1000(): {offerId: bigint, who: Uint8Array, nbOfBonds: bigint} {
        assert(this.isPicassoV1000)
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
    get isPicassoV1000(): boolean {
        return this._chain.getEventHash('BondedFinance.NewOffer') === 'a31df34b423037e305dbc2946d691428051e98fb362268dc0e78aff52ab30840'
    }

    /**
     * A new offer has been created.
     */
    get asPicassoV1000(): {offerId: bigint} {
        assert(this.isPicassoV1000)
        return this._chain.decodeEvent(this.event)
    }

    /**
     * A new offer has been created.
     */
    get isPicassoV1400(): boolean {
        return this._chain.getEventHash('BondedFinance.NewOffer') === '68b798e0fb8f433f37ecc5a1efa5af84a146a217c123fba86d358fdc60508217'
    }

    /**
     * A new offer has been created.
     */
    get asPicassoV1400(): {offerId: bigint, beneficiary: Uint8Array} {
        assert(this.isPicassoV1400)
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
    get isPicassoV1000(): boolean {
        return this._chain.getEventHash('BondedFinance.OfferCancelled') === 'a31df34b423037e305dbc2946d691428051e98fb362268dc0e78aff52ab30840'
    }

    /**
     * An offer has been cancelled by the `AdminOrigin`.
     */
    get asPicassoV1000(): {offerId: bigint} {
        assert(this.isPicassoV1000)
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
    get isPicassoV10005(): boolean {
        return this._chain.getEventHash('Pablo.LiquidityAdded') === '768cdd130e4e7cbfa742e476f2af6c5e7de4bdbf1f44e61e9be3626f6efa24c7'
    }

    /**
     * Liquidity added into the pool `T::PoolId`.
     */
    get asPicassoV10005(): {who: Uint8Array, poolId: bigint, assetAmounts: [bigint, bigint][], mintedLp: bigint} {
        assert(this.isPicassoV10005)
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
    get isPicassoV10005(): boolean {
        return this._chain.getEventHash('Pablo.LiquidityRemoved') === 'f83a7eb510fc980414891c8a407bd249e0662ff3a1e15034572f62a8a15540e5'
    }

    /**
     * Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
     */
    get asPicassoV10005(): {who: Uint8Array, poolId: bigint, assetAmounts: [bigint, bigint][]} {
        assert(this.isPicassoV10005)
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
    get isPicassoV10005(): boolean {
        return this._chain.getEventHash('Pablo.PoolCreated') === 'dac2b11b70d76f7d768871c6ed616e443b2aaf161355f79320a567e4059a9b0a'
    }

    /**
     * Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
     */
    get asPicassoV10005(): {poolId: bigint, owner: Uint8Array, assetWeights: [bigint, number][]} {
        assert(this.isPicassoV10005)
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
    get isPicassoV10005(): boolean {
        return this._chain.getEventHash('Pablo.Swapped') === 'e2cb97932583cb6d0722d9449b471d2ea8b363ac4580591664fe7471b8e463bb'
    }

    /**
     * Token exchange happened.
     */
    get asPicassoV10005(): {poolId: bigint, who: Uint8Array, baseAsset: bigint, quoteAsset: bigint, baseAmount: bigint, quoteAmount: bigint, fee: picassoV10005.Fee} {
        assert(this.isPicassoV10005)
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
    get isV10005(): boolean {
        return this._chain.getEventHash('StakingRewards.RewardPoolCreated') === '5e390e4b4872a6c90e625e09f7ad2ead01ad028359e113064d081b1ac6c45dc5'
    }

    /**
     * Pool with specified id `T::AssetId` was created successfully by `T::AccountId`.
     */
    get asV10005(): {poolId: bigint, owner: Uint8Array, poolConfig: v10005.RewardPoolConfiguration} {
        assert(this.isV10005)
        return this._chain.decodeEvent(this.event)
    }
}

export class StakingRewardsRewardPoolUpdatedEvent {
    private readonly _chain: Chain
    private readonly event: Event

    constructor(ctx: EventContext)
    constructor(ctx: ChainContext, event: Event)
    constructor(ctx: EventContext, event?: Event) {
        event = event || ctx.event
        assert(event.name === 'StakingRewards.RewardPoolUpdated')
        this._chain = ctx._chain
        this.event = event
    }

    get isV10005(): boolean {
        return this._chain.getEventHash('StakingRewards.RewardPoolUpdated') === 'a3a435ec02cce2bd18b336517dc3413dc0cd6cf40796e05e8a3178733e20bed4'
    }

    get asV10005(): {poolId: bigint, rewardUpdates: [bigint, v10005.RewardUpdate][]} {
        assert(this.isV10005)
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
    get isV10005(): boolean {
        return this._chain.getEventHash('StakingRewards.SplitPosition') === '8031e5788ebd4aef1e0b75ca8f52827e9667c64d25433e9e070ed74ba3f9a8e3'
    }

    /**
     * A staking position was split.
     */
    get asV10005(): {positions: [bigint, bigint, bigint][]} {
        assert(this.isV10005)
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

    get isV10005(): boolean {
        return this._chain.getEventHash('StakingRewards.StakeAmountExtended') === 'a41f82bf4e9ef7f7f630f7fb5696e3b2f4ca4baf7eb8af6a70d3faf535de3dc9'
    }

    get asV10005(): {fnftCollectionId: bigint, fnftInstanceId: bigint, amount: bigint} {
        assert(this.isV10005)
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

    get isV10005(): boolean {
        return this._chain.getEventHash('StakingRewards.Staked') === '51b801cf0907ec7517c5d002143ad3b27067de4f35b313be7a019444cfb926e0'
    }

    get asV10005(): {poolId: bigint, owner: Uint8Array, amount: bigint, durationPreset: bigint, fnftCollectionId: bigint, fnftInstanceId: bigint, rewardMultiplier: bigint, keepAlive: boolean} {
        assert(this.isV10005)
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

    get isV10005(): boolean {
        return this._chain.getEventHash('StakingRewards.Unstaked') === '49170892b4bc964636d4793ef5cf7eee1abd5fd1f34f55f4448294ba4a184c73'
    }

    get asV10005(): {owner: Uint8Array, fnftCollectionId: bigint, fnftInstanceId: bigint, slash: (bigint | undefined)} {
        assert(this.isV10005)
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
    get isPicassoV1000(): boolean {
        return this._chain.getEventHash('Vesting.Claimed') === '1f29af233c75b3b7d43d3ffbfe7da109a4f7c9f277896999fac76012939a6432'
    }

    /**
     * Claimed vesting. \[who, locked_amount\]
     */
    get asPicassoV1000(): {who: Uint8Array, asset: bigint, lockedAmount: bigint} {
        assert(this.isPicassoV1000)
        return this._chain.decodeEvent(this.event)
    }

    /**
     * Claimed vesting.
     */
    get isPicassoV10002(): boolean {
        return this._chain.getEventHash('Vesting.Claimed') === '1158bd677eb4e5aad57841bad2e35470c5be3bbc33b843378d69a8cf7bfced30'
    }

    /**
     * Claimed vesting.
     */
    get asPicassoV10002(): {who: Uint8Array, asset: bigint, vestingScheduleIds: picassoV10002.VestingScheduleIdSet, lockedAmount: bigint, claimedAmountPerSchedule: [bigint, bigint][]} {
        assert(this.isPicassoV10002)
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
    get isPicassoV1000(): boolean {
        return this._chain.getEventHash('Vesting.VestingScheduleAdded') === 'c5e29260a72cc5736d41a9413a02519d99775ae811581363c8cbdf2433143a79'
    }

    /**
     * Added new vesting schedule. \[from, to, schedule\]
     */
    get asPicassoV1000(): {from: Uint8Array, to: Uint8Array, asset: bigint, schedule: picassoV1000.VestingSchedule} {
        assert(this.isPicassoV1000)
        return this._chain.decodeEvent(this.event)
    }

    /**
     * Added new vesting schedule.
     */
    get isPicassoV10002(): boolean {
        return this._chain.getEventHash('Vesting.VestingScheduleAdded') === '76bb06af4efc9a40f5604bfe9dbe980d1cec79e966fe1f641bb9475c65a6808d'
    }

    /**
     * Added new vesting schedule.
     */
    get asPicassoV10002(): {from: Uint8Array, to: Uint8Array, asset: bigint, vestingScheduleId: bigint, schedule: picassoV10002.VestingSchedule, scheduleAmount: bigint} {
        assert(this.isPicassoV10002)
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
    get isPicassoV1000(): boolean {
        return this._chain.getEventHash('Vesting.VestingSchedulesUpdated') === 'b8a0d2208835f6ada60dd21cd93533d703777b3779109a7c6a2f26bad68c2f3b'
    }

    /**
     * Updated vesting schedules. \[who\]
     */
    get asPicassoV1000(): {who: Uint8Array} {
        assert(this.isPicassoV1000)
        return this._chain.decodeEvent(this.event)
    }
}
