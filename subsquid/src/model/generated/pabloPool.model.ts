import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, Index as Index_, OneToMany as OneToMany_} from "typeorm"
import * as marshal from "./marshal"
import {PabloPoolAsset} from "./pabloPoolAsset.model"
import {PabloTransaction} from "./pabloTransaction.model"

@Entity_()
export class PabloPool {
  constructor(props?: Partial<PabloPool>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  /**
   * ID of the last event that was used to derive this entity data
   */
  @Column_("text", {nullable: false})
  eventId!: string

  /**
   * Pool ID
   */
  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  poolId!: bigint

  @Column_("text", {nullable: false})
  owner!: string

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  lpIssued!: bigint

  @Column_("int4", {nullable: false})
  transactionCount!: number

  @Column_("text", {nullable: false})
  totalLiquidity!: string

  @Column_("text", {nullable: false})
  totalVolume!: string

  @Column_("text", {nullable: false})
  totalFees!: string

  @Column_("text", {nullable: false})
  baseAssetId!: string

  /**
   * Asset used for all quotes in this type
   */
  @Column_("text", {nullable: false})
  quoteAssetId!: string

  @Index_()
  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  blockNumber!: bigint

  /**
   * Unix timestamp in ms
   */
  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  calculatedTimestamp!: bigint

  @OneToMany_(() => PabloPoolAsset, e => e.pool)
  poolAssets!: PabloPoolAsset[]

  @OneToMany_(() => PabloTransaction, e => e.pool)
  transactions!: PabloTransaction[]
}
