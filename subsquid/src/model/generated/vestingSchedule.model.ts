import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, Index as Index_} from "typeorm"
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
   * Account that initiates the schedule
   */
  @Index_()
  @Column_("text", {nullable: false})
  from!: string

  /**
   * Chain event ID
   */
  @Index_()
  @Column_("text", {nullable: false})
  eventId!: string

  /**
   * Vesting schedule ID from chain
   */
  @Index_()
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
  @Column_("text", {nullable: false})
  assetId!: string

  /**
   * Vesting schedule
   */
  @Column_("jsonb", {transformer: {to: obj => obj.toJSON(), from: obj => new Schedule(undefined, marshal.nonNull(obj))}, nullable: false})
  schedule!: Schedule

  /**
   * Initial locked amount
   */
  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  totalAmount!: bigint

  /**
   * True if the schedule has been fully claimed
   */
  @Column_("bool", {nullable: false})
  fullyClaimed!: boolean
}
