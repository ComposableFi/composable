import assert from 'assert'
import {Chain, ChainContext, CallContext, Call, Result, Option} from './support'
import * as v10005 from './v10005'

export class PabloAddLiquidityCall {
    private readonly _chain: Chain
    private readonly call: Call

    constructor(ctx: CallContext)
    constructor(ctx: ChainContext, call: Call)
    constructor(ctx: CallContext, call?: Call) {
        call = call || ctx.call
        assert(call.name === 'Pablo.add_liquidity')
        this._chain = ctx._chain
        this.call = call
    }

    /**
     * Add liquidity to the given pool.
     * 
     * Emits `LiquidityAdded` event when successful.
     */
    get isV10005(): boolean {
        return this._chain.getCallHash('Pablo.add_liquidity') === '4136e1086cde2dce1bb30bea9f1916a910219e0e687492936eb014a64f7ae4f5'
    }

    /**
     * Add liquidity to the given pool.
     * 
     * Emits `LiquidityAdded` event when successful.
     */
    get asV10005(): {poolId: bigint, assets: [bigint, bigint][], minMintAmount: bigint, keepAlive: boolean} {
        assert(this.isV10005)
        return this._chain.decodeCall(this.call)
    }
}

export class PabloBuyCall {
    private readonly _chain: Chain
    private readonly call: Call

    constructor(ctx: CallContext)
    constructor(ctx: ChainContext, call: Call)
    constructor(ctx: CallContext, call?: Call) {
        call = call || ctx.call
        assert(call.name === 'Pablo.buy')
        this._chain = ctx._chain
        this.call = call
    }

    /**
     * Execute a buy order on pool.
     * 
     * Emits `Swapped` event when successful.
     */
    get isV10005(): boolean {
        return this._chain.getCallHash('Pablo.buy') === '2a30bc2ea7b0df6399125fe6e21cb61c7f1d4a30ed1dc65fedb58a0b044d089b'
    }

    /**
     * Execute a buy order on pool.
     * 
     * Emits `Swapped` event when successful.
     */
    get asV10005(): {poolId: bigint, inAssetId: bigint, outAsset: v10005.AssetAmount, keepAlive: boolean} {
        assert(this.isV10005)
        return this._chain.decodeCall(this.call)
    }
}

export class PabloRemoveLiquidityCall {
    private readonly _chain: Chain
    private readonly call: Call

    constructor(ctx: CallContext)
    constructor(ctx: ChainContext, call: Call)
    constructor(ctx: CallContext, call?: Call) {
        call = call || ctx.call
        assert(call.name === 'Pablo.remove_liquidity')
        this._chain = ctx._chain
        this.call = call
    }

    /**
     * Remove liquidity from the given pool.
     * 
     * Emits `LiquidityRemoved` event when successful.
     */
    get isV10005(): boolean {
        return this._chain.getCallHash('Pablo.remove_liquidity') === '82220fa2492d152a12c58b629f8992fa0e0ae1ae901992cc22900edcde815170'
    }

    /**
     * Remove liquidity from the given pool.
     * 
     * Emits `LiquidityRemoved` event when successful.
     */
    get asV10005(): {poolId: bigint, lpAmount: bigint, minReceive: [bigint, bigint][]} {
        assert(this.isV10005)
        return this._chain.decodeCall(this.call)
    }
}

export class PabloSwapCall {
    private readonly _chain: Chain
    private readonly call: Call

    constructor(ctx: CallContext)
    constructor(ctx: ChainContext, call: Call)
    constructor(ctx: CallContext, call?: Call) {
        call = call || ctx.call
        assert(call.name === 'Pablo.swap')
        this._chain = ctx._chain
        this.call = call
    }

    /**
     * Execute a specific swap operation.
     * 
     * The `quote_amount` is always the quote asset amount (A/B => B), (B/A => A).
     * 
     * Emits `Swapped` event when successful.
     */
    get isV10005(): boolean {
        return this._chain.getCallHash('Pablo.swap') === 'f8750a9705ddc0725a912b5758059b5b0d0e9745b9bb777a05086907cdeb1f8e'
    }

    /**
     * Execute a specific swap operation.
     * 
     * The `quote_amount` is always the quote asset amount (A/B => B), (B/A => A).
     * 
     * Emits `Swapped` event when successful.
     */
    get asV10005(): {poolId: bigint, inAsset: v10005.AssetAmount, minReceive: v10005.AssetAmount, keepAlive: boolean} {
        assert(this.isV10005)
        return this._chain.decodeCall(this.call)
    }
}
