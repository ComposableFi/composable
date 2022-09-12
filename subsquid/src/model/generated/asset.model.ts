import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, Index as Index_, OneToMany as OneToMany_} from "typeorm"
import * as marshal from "./marshal"
import {HistoricalAssetPrice} from "./historicalAssetPrice.model"

@Entity_()
export class Asset {
  constructor(props?: Partial<Asset>) {
    Object.assign(this, props)
  }

  /**
   * ID of the asset in Picasso
   */
  @PrimaryColumn_()
  id!: string

  /**
   * ID of the event that added the last price
   */
  @Index_()
  @Column_("text", {nullable: false})
  eventId!: string

  /**
   * Latest price in USD
   */
  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  price!: bigint

  @OneToMany_(() => HistoricalAssetPrice, e => e.asset)
  historicalPrices!: HistoricalAssetPrice[]
}
