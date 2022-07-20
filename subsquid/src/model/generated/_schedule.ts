import assert from "assert"
import * as marshal from "./marshal"
import {ScheduleWindow} from "./_scheduleWindow"

export class Schedule {
  private _window!: ScheduleWindow
  private _periodCount!: bigint
  private _perPeriod!: bigint

  constructor(props?: Partial<Omit<Schedule, 'toJSON'>>, json?: any) {
    Object.assign(this, props)
    if (json != null) {
      this._window = new ScheduleWindow(undefined, marshal.nonNull(json.window))
      this._periodCount = marshal.bigint.fromJSON(json.periodCount)
      this._perPeriod = marshal.bigint.fromJSON(json.perPeriod)
    }
  }

  get window(): ScheduleWindow {
    assert(this._window != null, 'uninitialized access')
    return this._window
  }

  set window(value: ScheduleWindow) {
    this._window = value
  }

  get periodCount(): bigint {
    assert(this._periodCount != null, 'uninitialized access')
    return this._periodCount
  }

  set periodCount(value: bigint) {
    this._periodCount = value
  }

  get perPeriod(): bigint {
    assert(this._perPeriod != null, 'uninitialized access')
    return this._perPeriod
  }

  set perPeriod(value: bigint) {
    this._perPeriod = value
  }

  toJSON(): object {
    return {
      window: this.window.toJSON(),
      periodCount: marshal.bigint.toJSON(this.periodCount),
      perPeriod: marshal.bigint.toJSON(this.perPeriod),
    }
  }
}
