import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_} from "typeorm"
import * as marshal from "./marshal"
import {PicassoTransactionType} from "./_picassoTransactionType"

@Entity_()
export class PicassoTransaction {
  constructor(props?: Partial<PicassoTransaction>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  /**
   * ID of the event that was used to derive this transaction
   */
  @Column_("text", {nullable: false})
  eventId!: string

  /**
   * ID of account that executed transaction
   */
  @Column_("text", {nullable: false})
  accountId!: string

  @Column_("varchar", {length: 37, nullable: false})
  transactionType!: PicassoTransactionType

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  blockNumber!: bigint

  @Column_("timestamp with time zone", {nullable: false})
  date!: Date
}
