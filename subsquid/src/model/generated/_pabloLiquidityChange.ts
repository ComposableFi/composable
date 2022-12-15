import assert from "assert"
import * as marshal from "./marshal"

export class PabloLiquidityChange {
    private _id!: string
    private _assetId!: string
    private _amount!: bigint

    constructor(props?: Partial<Omit<PabloLiquidityChange, 'toJSON'>>, json?: any) {
        Object.assign(this, props)
        if (json != null) {
            this._id = marshal.id.fromJSON(json.id)
            this._assetId = marshal.string.fromJSON(json.assetId)
            this._amount = marshal.bigint.fromJSON(json.amount)
        }
    }

    get id(): string {
        assert(this._id != null, 'uninitialized access')
        return this._id
    }

    set id(value: string) {
        this._id = value
    }

    get assetId(): string {
        assert(this._assetId != null, 'uninitialized access')
        return this._assetId
    }

    set assetId(value: string) {
        this._assetId = value
    }

    get amount(): bigint {
        assert(this._amount != null, 'uninitialized access')
        return this._amount
    }

    set amount(value: bigint) {
        this._amount = value
    }

    toJSON(): object {
        return {
            id: this.id,
            assetId: this.assetId,
            amount: marshal.bigint.toJSON(this.amount),
        }
    }
}
