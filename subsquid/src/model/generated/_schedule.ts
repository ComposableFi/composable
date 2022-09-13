import assert from "assert"
import * as marshal from "./marshal"
import {ScheduleWindow} from "./_scheduleWindow"

export class Schedule {
  private _vestingScheduleId!: bigint
  private _window!: ScheduleWindow
  private _periodCount!: bigint
  private _perPeriod!: bigint
  private _alreadyClaimed!: bigint

  constructor(props?: Partial<Omit<Schedule, 'toJSON'>>, json?: any) {
    Object.assign(this, props)
    if (json != null) {
      this._vestingScheduleId = marshal.bigint.fromJSON(json.vestingScheduleId)
      this._window = new ScheduleWindow(undefined, marshal.nonNull(json.window))
      this._periodCount = marshal.bigint.fromJSON(json.periodCount)
      this._perPeriod = marshal.bigint.fromJSON(json.perPeriod)
      this._alreadyClaimed = marshal.bigint.fromJSON(json.alreadyClaimed)
    }
  }

  get vestingScheduleId(): bigint {
    assert(this._vestingScheduleId != null, 'uninitialized access')
    return this._vestingScheduleId
  }

  set vestingScheduleId(value: bigint) {
    this._vestingScheduleId = value
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

  get alreadyClaimed(): bigint {
    assert(this._alreadyClaimed != null, 'uninitialized access')
    return this._alreadyClaimed
  }

  set alreadyClaimed(value: bigint) {
    this._alreadyClaimed = value
  }

  toJSON(): object {
    return {
      vestingScheduleId: marshal.bigint.toJSON(this.vestingScheduleId),
      window: this.window.toJSON(),
      periodCount: marshal.bigint.toJSON(this.periodCount),
      perPeriod: marshal.bigint.toJSON(this.perPeriod),
      alreadyClaimed: marshal.bigint.toJSON(this.alreadyClaimed),
    }
  }
}
