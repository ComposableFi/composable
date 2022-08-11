import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {PicassoAccount} from "./picassoAccount.model"
import {PicassoTransactionType} from "./_picassoTransactionType"

@Entity_()
export class PicassoTransaction {
  constructor(props?: Partial<PicassoTransaction>) {
    Object.assign(this, props)
  }

  /**
   * ID of the event that was used to derive this transaction
   */
  @PrimaryColumn_()
  id!: string

  @Column_("text", {nullable: false})
  eventId!: string

  @Column_("text", {nullable: false})
  transactionId!: string

  @Index_()
  @ManyToOne_(() => PicassoAccount, {nullable: false})
  who!: PicassoAccount

  @Column_("varchar", {length: 37, nullable: false})
  transactionType!: PicassoTransactionType

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  blockNumber!: bigint

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  fee!: bigint

  /**
   * Unix timestamp in ms
   */
  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  timestamp!: bigint
}
