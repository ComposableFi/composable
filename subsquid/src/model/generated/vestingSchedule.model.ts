import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_} from "typeorm"
import * as marshal from "./marshal"
import {Schedule} from "./_schedule"

@Entity_()
export class VestingSchedule {
  constructor(props?: Partial<VestingSchedule>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  /**
   * account that initiates the schedule
   */
  @Column_("text", {nullable: false})
  from!: string

  /**
   * chain event ID
   */
  @Column_("text", {nullable: false})
  eventId!: string

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  scheduleId!: bigint

  /**
   * 'To' account for the vesting schedule
   */
  @Column_("text", {nullable: false})
  to!: string

  /**
   * Asset ID
   */
  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  asset!: bigint

  /**
   * Vesting schedule
   */
  @Column_("jsonb", {transformer: {to: obj => obj.toJSON(), from: obj => new Schedule(undefined, marshal.nonNull(obj))}, nullable: false})
  schedule!: Schedule
}
