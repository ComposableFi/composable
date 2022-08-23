import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {Asset} from "./asset.model"

@Entity_()
export class HistoricalAssetPrice {
  constructor(props?: Partial<HistoricalAssetPrice>) {
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
   * ID of the asset
   */
  @Index_()
  @ManyToOne_(() => Asset, {nullable: false})
  asset!: Asset

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  price!: bigint

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  timestamp!: bigint
}
