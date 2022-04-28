import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {PabloPool} from "./pabloPool.model"
import {PabloTransactionType} from "./_pabloTransactionType"

@Entity_()
export class PabloTransaction {
  constructor(props?: Partial<PabloTransaction>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  /**
   * ID of the event that was used to derive this transaction
   */
  @Column_("text", {nullable: false})
  eventId!: string

  @Index_()
  @ManyToOne_(() => PabloPool, {nullable: false})
  pool!: PabloPool

  @Column_("text", {nullable: false})
  who!: string

  @Column_("varchar", {length: 16, nullable: true})
  transactionType!: PabloTransactionType | undefined | null

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  baseAssetId!: bigint

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  baseAssetAmount!: bigint

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  quoteAssetId!: bigint

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  quoteAssetAmount!: bigint

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  blockNumber!: bigint

  @Column_("text", {nullable: false})
  priceInQuoteAsset!: string

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  receivedTimestamp!: bigint
}
