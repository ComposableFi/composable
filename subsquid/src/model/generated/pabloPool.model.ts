import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, OneToMany as OneToMany_} from "typeorm"
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
   * Pool ID
   */
  @Column_("text", {nullable: false})
  poolId!: string

  @Column_("text", {nullable: false})
  owner!: string

  @Column_("integer", {nullable: false})
  transactionCount!: number

  @Column_("text", {nullable: false})
  totalLiquidity!: string

  @Column_("text", {nullable: false})
  totalVolume!: string

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  quoteAssetId!: bigint

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  blockNumber!: bigint

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  calculatedTimestamp!: bigint

  @OneToMany_(() => PabloPoolAsset, e => e.pool)
  poolAssets!: PabloPoolAsset[]

  @OneToMany_(() => PabloTransaction, e => e.pool)
  transactions!: PabloTransaction[]
}
