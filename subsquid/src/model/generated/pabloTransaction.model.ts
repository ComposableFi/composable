import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, OneToOne as OneToOne_, Index as Index_, JoinColumn as JoinColumn_, ManyToOne as ManyToOne_} from "typeorm"
import * as marshal from "./marshal"
import {Event} from "./event.model"
import {PabloPool} from "./pabloPool.model"

@Entity_()
export class PabloTransaction {
  constructor(props?: Partial<PabloTransaction>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  @Index_({unique: true})
  @OneToOne_(() => Event, {nullable: false})
  @JoinColumn_()
  event!: Event

  @Index_()
  @ManyToOne_(() => PabloPool, {nullable: true})
  pool!: PabloPool

  @Column_("text", {nullable: false})
  baseAssetId!: string

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  baseAssetAmount!: bigint

  @Column_("text", {nullable: false})
  quoteAssetId!: string

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  quoteAssetAmount!: bigint

  @Column_("text", {nullable: false})
  spotPrice!: string

  /**
   * Optional: Only certain transaction types have fees charged by Pablo. Does NOT include the collected extrinsic execution fee.
   */
  @Column_("text", {nullable: false})
  fee!: string
}
