import assert from "assert"
import * as marshal from "./marshal"
import {Event} from "./event.model"
import {PabloPool} from "./pabloPool.model"

export class PabloTransactionSwap {
    public readonly isTypeOf = 'PabloTransactionSwap'
    private _id!: string
    private _event!: string
    private _pool!: string
    private _baseAssetId!: string
    private _baseAssetAmount!: bigint
    private _quoteAssetId!: string
    private _quoteAssetAmount!: bigint
    private _spotPrice!: bigint
    private _fee!: string

    constructor(props?: Partial<Omit<PabloTransactionSwap, 'toJSON'>>, json?: any) {
        Object.assign(this, props)
        if (json != null) {
            this._id = marshal.id.fromJSON(json.id)
            this._event = marshal.string.fromJSON(json.event)
            this._pool = marshal.string.fromJSON(json.pool)
            this._baseAssetId = marshal.string.fromJSON(json.baseAssetId)
            this._baseAssetAmount = marshal.bigint.fromJSON(json.baseAssetAmount)
            this._quoteAssetId = marshal.string.fromJSON(json.quoteAssetId)
            this._quoteAssetAmount = marshal.bigint.fromJSON(json.quoteAssetAmount)
            this._spotPrice = marshal.bigint.fromJSON(json.spotPrice)
            this._fee = marshal.string.fromJSON(json.fee)
        }
    }

    get id(): string {
        assert(this._id != null, 'uninitialized access')
        return this._id
    }

    set id(value: string) {
        this._id = value
    }

    get event(): string {
        assert(this._event != null, 'uninitialized access')
        return this._event
    }

    set event(value: string) {
        this._event = value
    }

    get pool(): string {
        assert(this._pool != null, 'uninitialized access')
        return this._pool
    }

    set pool(value: string) {
        this._pool = value
    }

    get baseAssetId(): string {
        assert(this._baseAssetId != null, 'uninitialized access')
        return this._baseAssetId
    }

    set baseAssetId(value: string) {
        this._baseAssetId = value
    }

    get baseAssetAmount(): bigint {
        assert(this._baseAssetAmount != null, 'uninitialized access')
        return this._baseAssetAmount
    }

    set baseAssetAmount(value: bigint) {
        this._baseAssetAmount = value
    }

    get quoteAssetId(): string {
        assert(this._quoteAssetId != null, 'uninitialized access')
        return this._quoteAssetId
    }

    set quoteAssetId(value: string) {
        this._quoteAssetId = value
    }

    get quoteAssetAmount(): bigint {
        assert(this._quoteAssetAmount != null, 'uninitialized access')
        return this._quoteAssetAmount
    }

    set quoteAssetAmount(value: bigint) {
        this._quoteAssetAmount = value
    }

    get spotPrice(): bigint {
        assert(this._spotPrice != null, 'uninitialized access')
        return this._spotPrice
    }

    set spotPrice(value: bigint) {
        this._spotPrice = value
    }

    /**
     * Does NOT include the collected extrinsic execution fee.
     */
    get fee(): string {
        assert(this._fee != null, 'uninitialized access')
        return this._fee
    }

    set fee(value: string) {
        this._fee = value
    }

    toJSON(): object {
        return {
            isTypeOf: this.isTypeOf,
            id: this.id,
            event: this.event,
            pool: this.pool,
            baseAssetId: this.baseAssetId,
            baseAssetAmount: marshal.bigint.toJSON(this.baseAssetAmount),
            quoteAssetId: this.quoteAssetId,
            quoteAssetAmount: marshal.bigint.toJSON(this.quoteAssetAmount),
            spotPrice: marshal.bigint.toJSON(this.spotPrice),
            fee: this.fee,
        }
    }
}
