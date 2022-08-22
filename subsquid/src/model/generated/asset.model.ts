import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, OneToMany as OneToMany_} from "typeorm"
import {HistoricalAssetPrice} from "./historicalAssetPrice.model"

@Entity_()
export class Asset {
  constructor(props?: Partial<Asset>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  /**
   * ID of the event that added the last price
   */
  @Column_("text", {nullable: false})
  eventId!: string

  /**
   * ID of the asset
   */
  @Column_("text", {nullable: false})
  assetId!: string

  @OneToMany_(() => HistoricalAssetPrice, e => e.asset)
  historicalPrices!: HistoricalAssetPrice[]
}
