import assert from "assert"
import * as marshal from "./marshal"

export class ScheduleWindow {
    private _start!: bigint
    private _period!: bigint
    private _kind!: string

    constructor(props?: Partial<Omit<ScheduleWindow, 'toJSON'>>, json?: any) {
        Object.assign(this, props)
        if (json != null) {
            this._start = marshal.bigint.fromJSON(json.start)
            this._period = marshal.bigint.fromJSON(json.period)
            this._kind = marshal.string.fromJSON(json.kind)
        }
    }

    get start(): bigint {
        assert(this._start != null, 'uninitialized access')
        return this._start
    }

    set start(value: bigint) {
        this._start = value
    }

    get period(): bigint {
        assert(this._period != null, 'uninitialized access')
        return this._period
    }

    set period(value: bigint) {
        this._period = value
    }

    get kind(): string {
        assert(this._kind != null, 'uninitialized access')
        return this._kind
    }

    set kind(value: string) {
        this._kind = value
    }

    toJSON(): object {
        return {
            start: marshal.bigint.toJSON(this.start),
            period: marshal.bigint.toJSON(this.period),
            kind: this.kind,
        }
    }
}
