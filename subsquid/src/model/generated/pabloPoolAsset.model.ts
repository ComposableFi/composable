import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {PabloPool} from "./pabloPool.model"

@Entity_()
export class PabloPoolAsset {
  constructor(props?: Partial<PabloPoolAsset>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  @Column_("text", {nullable: false})
  assetId!: string

  @Index_()
  @ManyToOne_(() => PabloPool, {nullable: true})
  pool!: PabloPool

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  totalLiquidity!: bigint

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  totalVolume!: bigint

  @Index_()
  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  blockNumber!: bigint

  /**
   * Unix timestamp in ms
   */
  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  calculatedTimestamp!: bigint
}
