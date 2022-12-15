import assert from "assert"
import * as marshal from "./marshal"
import {Event} from "./event.model"
import {PabloPool} from "./pabloPool.model"
import {PabloPoolType} from "./_pabloPoolType"

export class PabloTransactionCreatePool {
    public readonly isTypeOf = 'PabloTransactionCreatePool'
    private _id!: string
    private _event!: string
    private _pool!: string
    private _poolType!: PabloPoolType

    constructor(props?: Partial<Omit<PabloTransactionCreatePool, 'toJSON'>>, json?: any) {
        Object.assign(this, props)
        if (json != null) {
            this._id = marshal.id.fromJSON(json.id)
            this._event = marshal.string.fromJSON(json.event)
            this._pool = marshal.string.fromJSON(json.pool)
            this._poolType = marshal.enumFromJson(json.poolType, PabloPoolType)
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

    get poolType(): PabloPoolType {
        assert(this._poolType != null, 'uninitialized access')
        return this._poolType
    }

    set poolType(value: PabloPoolType) {
        this._poolType = value
    }

    toJSON(): object {
        return {
            isTypeOf: this.isTypeOf,
            id: this.id,
            event: this.event,
            pool: this.pool,
            poolType: this.poolType,
        }
    }
}
