import assert from "assert"
import * as marshal from "./marshal"
import {Event} from "./event.model"
import {PabloPool} from "./pabloPool.model"
import {PabloLiquidityChange} from "./_pabloLiquidityChange"

export class PabloTransactionAddLiquidity {
    public readonly isTypeOf = 'PabloTransactionAddLiquidity'
    private _id!: string
    private _event!: string
    private _pool!: string
    private _liquidityChanges!: (PabloLiquidityChange)[] | undefined | null

    constructor(props?: Partial<Omit<PabloTransactionAddLiquidity, 'toJSON'>>, json?: any) {
        Object.assign(this, props)
        if (json != null) {
            this._id = marshal.id.fromJSON(json.id)
            this._event = marshal.string.fromJSON(json.event)
            this._pool = marshal.string.fromJSON(json.pool)
            this._liquidityChanges = json.liquidityChanges == null ? undefined : marshal.fromList(json.liquidityChanges, val => new PabloLiquidityChange(undefined, marshal.nonNull(val)))
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

    get liquidityChanges(): (PabloLiquidityChange)[] | undefined | null {
        return this._liquidityChanges
    }

    set liquidityChanges(value: (PabloLiquidityChange)[] | undefined | null) {
        this._liquidityChanges = value
    }

    toJSON(): object {
        return {
            isTypeOf: this.isTypeOf,
            id: this.id,
            event: this.event,
            pool: this.pool,
            liquidityChanges: this.liquidityChanges == null ? undefined : this.liquidityChanges.map((val: any) => val.toJSON()),
        }
    }
}
