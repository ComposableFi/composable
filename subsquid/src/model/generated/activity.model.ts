import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_} from "typeorm"
import * as marshal from "./marshal"

@Entity_()
export class Activity {
  constructor(props?: Partial<Activity>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  /**
   * ID of the event associated with this activity
   */
  @Column_("text", {nullable: false})
  eventId!: string

  /**
   * ID of the transaction associated with this activity
   */
  @Column_("text", {nullable: false})
  transactionId!: string

  /**
   * ID of the active account
   */
  @Column_("text", {nullable: false})
  accountId!: string

  /**
   * Timestamp of the block in which this activity occurred
   */
  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  timestamp!: bigint
}
